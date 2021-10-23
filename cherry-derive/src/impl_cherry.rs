use proc_macro::TokenStream;
use std::str::FromStr;

use heck::SnakeCase;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Ident, Lit, Meta, NestedMeta, punctuated::Punctuated};

pub fn derive(ast: syn::DeriveInput) -> TokenStream {
    let (db, table) = parse_attrs(&ast);
    let dbname = db.to_lowercase();
    let ident = ast.ident;

    let fields = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Cherry only impl for struct."),
    };

    let fields_vec = fields.iter().filter_map(|field|
        field.ident.as_ref().map(|ident| ident.to_string())
    ).collect::<Vec<String>>();

    let fields_str = fields_vec.iter().map(|s|
        format!(r#" "{}", "#, s)
    ).collect::<String>();

    let build_arguments_str = fields_vec.iter().map(|s|
        format!(r#".add(&self.{}) "#, s)
    ).collect::<String>();
    let arguments_type = match dbname.as_str() {
        "mysql" => "MySqlArguments",
        "postgres" => "PgArguments",
        "sqlite" => "SqliteArguments",
        "mssql" => "MssqlArguments",
        "any" => "AnyArguments",
        _ => "UnSupportArguments"
    };

    let from_row_str = fields_vec.iter().map(|s|
        format!(r#"{0}: rows.decode_{1}("{0}")?, "#, s, dbname)
    ).collect::<String>();

    let token = quote!(
        impl cherry::Cherry for #ident {
            fn table() -> &'static str {
                #table
            }
            fn columns() -> Vec<&'static str> {
                vec![ [fields_str] ]
            }

            fn to_arguments(&self) -> cherry::WrapArguments {
                use cherry::Arguments;
                let mut arg = cherry::[arguments_type]::new();
                arg[build_arguments_str];
                cherry::WrapArguments::[arguments_type](arg)
            }

            fn arguments<'a>(&'a self, arguments: &mut cherry::WrapArguments<'a>) {
                use cherry::Arguments;
                match arguments {
                    cherry::WrapArguments::[arguments_type](a) => {
                        a [build_arguments_str] ;
                    }
                    _ => panic!("Database arguments type mismatch.")
                }
            }

            fn from_row(rows: &cherry::WrapRows) -> Result<Self, anyhow::Error> {
                let x = Self {
                    [from_row_str]
                };
                Ok(x)
            }
        }
    );

    let token = token.to_string()
        // .replace("[db]", db.as_str())
        .replace("[fields_str]", fields_str.as_str())
        .replace("[from_row_str]", from_row_str.as_str())
        .replace("[build_arguments_str]", build_arguments_str.as_str())
        .replace("[arguments_type]", arguments_type);

    // println!("{}", token);

    TokenStream::from_str(token.as_str()).expect("Parse token stream failed")
}

static DB_NAMES: [&str; 4] = ["mysql", "postgres", "mssql", "sqlite", ]; // "any"

fn parse_attrs(ast: &syn::DeriveInput) -> (String, String) {
    let msg = format!("Only support: {:?}", DB_NAMES).replace("\"", "");
    ast.attrs.iter().find_map(|attr| {
        if let Meta::List(meta_list) = attr.parse_meta().unwrap() {
            if meta_list.path.get_ident() ==
                Some(&Ident::new("cherry", Span::call_site())) {
                let (db_name, table) = extract_attrs(&meta_list.nested);
                let db_name = db_name.expect(msg.as_str());
                if !DB_NAMES.contains(&db_name.as_str()) {
                    panic!("{}", msg);
                }

                let table = table.unwrap_or_else(||
                    (&ast.ident).to_string().to_snake_case());
                return Some((db_name, table));
            }
        }
        None
    }).expect(msg.as_str())
}

fn extract_attrs<P>(props: &Punctuated<NestedMeta, P>) -> (Option<String>, Option<String>) {
    let db_name = match props.first() {
        Some(NestedMeta::Meta(Meta::Path(p))) => Some(p.to_token_stream().to_string()),
        _ => None
    };

    let table = props.iter().find_map(|item| {
        if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
            if name_value.path.get_ident() ==
                Some(&Ident::new("table", Span::call_site())){
                if let Lit::Str(s) = &name_value.lit {
                    return Some(s.value());
                }
            }
        }
        None
    });

    (db_name, table)
}
