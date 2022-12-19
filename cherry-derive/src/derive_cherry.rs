use proc_macro::TokenStream;
use std::collections::HashMap;
use std::str::FromStr;

use heck::SnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, Ident, Lit, Meta, NestedMeta, punctuated::Punctuated};

pub fn derive(ast: syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let mut attrs = parse_attrs(&ast);
    let table = attrs.remove("table").unwrap_or(ident.to_string().to_snake_case());
    let db_values = attrs.remove("database").unwrap_or_default();
    let mut tokens: Vec<String> = vec![];

    for db_name in get_databases(&db_values) {
        let token = quote!(
            impl<'a> cherry::Cherry<'a, [db_type], [arguments_type]> for #ident {
                fn table() -> &'static str {
                    #table
                }
                fn columns() -> Vec<(&'static str, &'static str)> {
                    vec![ [fields] ]
                }

                fn arguments(&'a self, arguments: &mut [arguments_type]) {
                    [arguments]
                }

                fn from_row(row: &<[db_type] as cherry::sqlx::Database>::Row) -> Result<Self, cherry::Error> {
                    use cherry::sqlx::Row;
                    Ok( Self { [from_row] } )
                }
            }
        );

        tokens.push(replace(token.to_string(), &db_name, &ast));
    }

    TokenStream::from_str(tokens.join("\n").as_str())
        .expect("Parse token stream failed")
}

fn replace(token: String, db_name: &str, ast: &syn::DeriveInput) -> String {
    let fields = match &ast.data {
        Data::Struct(ref s) => &s.fields,
        _ => panic!("Cherry only allow impl for struct."),
    };

    let fields_str = fields.iter().filter_map(|field|
        field.ident.as_ref().map(|ident| ident.to_string())
    ).collect::<Vec<String>>();

    let fields = fields_str.iter().map(|s|
        format!(r#" ("{0}", "{0}"), "#, s)
    ).collect::<String>();

    let arguments = fields_str.iter().map(|s|
        format!(r#" arguments.add(&self.{}); "#, s)
    ).collect::<String>();

    let from_row = fields_str.iter().map(|s|
        format!(r#" {0}: row.try_get("{0}")?, "#, s)
    ).collect::<String>();

    token
        .replace("[db_type]", database_type(db_name))
        .replace("[arguments_type]", arguments_type(db_name))
        .replace("[fields]", fields.as_str())
        .replace("[arguments]", arguments.as_str())
        .replace("[from_row]", from_row.as_str())
}

fn get_databases(prop: &str) -> Vec<String> {
    let mut values = prop.split(',')
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().to_string())
        .collect::<Vec<String>>();

    if values.is_empty() {
        #[cfg(feature = "sqlite")] values.push("sqlite".into());
        #[cfg(feature = "mysql")] values.push("mysql".into());
        #[cfg(feature = "postgres")] values.push("postgres".into());
        #[cfg(feature = "mssql")] values.push("mssql".into());
    }

    values
}

fn database_type(db_name: &str) -> &'static str {
    match db_name {
        "sqlite" => "cherry::sqlx::Sqlite",
        "mysql" => "cherry::sqlx::MySql",
        "postgres" => "cherry::sqlx::Postgres",
        _ => panic!("Unknown database `{}`", db_name),
    }
}

fn arguments_type(db_name: &str) -> &'static str {
    match db_name {
        "sqlite" => "cherry::sqlite::SqliteArguments<'a>",
        "mysql" => "cherry::mysql::MySqlArguments",
        "postgres" => "cherry::postgres::PgArguments",
        _ => panic!("Unknown arguments database `{}`", db_name),
    }
}

fn parse_attrs(ast: &syn::DeriveInput) -> HashMap<String, String> {
    ast.attrs.iter().find_map(|attr| {
        match attr.parse_meta().unwrap() {
            Meta::List(meta_list) => Some(meta_list),
            _ => None
        }
    }).filter(|meta_list| {
        meta_list.path.get_ident() == Some(&Ident::new("cherry", Span::call_site()))
    }).map(|meta_list| {
        // extract_attrs(&meta_list.nested)
        props(&meta_list.nested)
    }).unwrap_or_default()

    // match value {
    //     Some(Some(table)) => table,
    //     Some(_) => panic!("Unknown attribute."),
    //     // The `cherry` attribute not exists. Pick the default name.
    //     _ => ast.ident.to_string().to_snake_case(),
    // }
}

fn props<P>(props: &Punctuated<NestedMeta, P>) -> HashMap<String, String> {
    props.iter().filter_map(|v| match v {
        NestedMeta::Meta(Meta::NameValue(value)) => Some(value),
        _ => None
    }).map(|v| {
        let key = v.path.get_ident().map(|ident| ident.to_string());
        let value = match &v.lit {
            Lit::Str(s) => Some(s.value()),
            _ => None,
        };
        (key, value)
    }).filter_map(|(k, v)|
        match k.is_some() && v.is_some() {
            true => Some((k.unwrap(), v.unwrap())),
            _ => None,
        }
    ).collect()
}


/*
fn extract_attrs<P>(props: &Punctuated<NestedMeta, P>) -> Option<String> {
    if props.len() > 1 {
        return None;
    }
    let lit = match props.first() {
        Some(NestedMeta::Meta(Meta::NameValue(value))) => Some(value),
        _ => None
    }.filter(|value| {
        value.path.get_ident() == Some(&Ident::new("table", Span::call_site()))
    }).map(|value| &value.lit);
    match lit {
        Some(Lit::Str(s)) => Some(s.value()),
        _ => None
    }
}
*/

/*
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
*/