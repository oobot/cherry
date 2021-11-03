use proc_macro::TokenStream;
use std::str::FromStr;

use heck::SnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, Ident, Lit, Meta, NestedMeta, punctuated::Punctuated};

pub fn derive(ast: syn::DeriveInput) -> TokenStream {
    let fields = match &ast.data {
        Data::Struct(ref s) => &s.fields,
        _ => panic!("Cherry only impl for struct."),
    };
    let ident = &ast.ident;
    let table = parse_attrs(&ast);

    let fields_vec = fields.iter().filter_map(|field|
        field.ident.as_ref().map(|ident| ident.to_string())
    ).collect::<Vec<String>>();

    let fields = fields_vec.iter().map(|s|
        format!(r#" "{}", "#, s)
    ).collect::<String>();

    let arguments = fields_vec.iter().map(|s|
        format!(r#" arguments.add(&self.{}); "#, s)
    ).collect::<String>();

    let from_row = fields_vec.iter().map(|s|
        format!(r#" {0}: row.try_get("{0}")?, "#, s)
    ).collect::<String>();

    let token = quote!(
        impl cherry::Cherry for #ident {
            fn table() -> &'static str {
                #table
            }
            fn columns() -> Vec<&'static str> {
                vec![ [fields] ]
            }

            fn arguments<'a>(&'a self, arguments: &mut cherry::types::Arguments<'a>) {
                use cherry::sqlx::Arguments as OtherArguments;
                [arguments]
            }

            fn from_row(row: &cherry::types::Row) -> Result<Self, cherry::error::Error> {
                use cherry::sqlx::Row as OtherRow;
                Ok( Self { [from_row] } )
            }
        }
    );

    let token = token.to_string()
        .replace("[fields]", fields.as_str())
        .replace("[arguments]", arguments.as_str())
        .replace("[from_row]", from_row.as_str());

    TokenStream::from_str(token.as_str()).expect("Parse token stream failed")
}

// If attribute exists, should set `table` value, and only `table` allowed.
fn parse_attrs(ast: &syn::DeriveInput) -> String {
    let value = ast.attrs.iter().find_map(|attr| {
        match attr.parse_meta().unwrap() {
            Meta::List(meta_list) => Some(meta_list),
            _ => None
        }
    }).filter(|meta_list| {
        meta_list.path.get_ident() == Some(&Ident::new("cherry", Span::call_site()))
    }).map(|meta_list| {
        extract_attrs(&meta_list.nested)
    });

    match value {
        Some(Some(table)) => table,
        Some(_) => panic!("Unknown attribute."),
        // The `cherry` attribute not exists. Pick the default name.
        _ => ast.ident.to_string().to_snake_case(),
    }

    // ast.attrs.iter().find_map(|attr| {
    //     if let Meta::List(meta_list) = attr.parse_meta().unwrap() {
    //         if meta_list.path.get_ident() ==
    //             Some(&Ident::new("cherry", Span::call_site())) {
    //
    //             let table = extract_attrs(&meta_list.nested)
    //                 .unwrap_or((&ast.ident.to_string().to_snake_case()).clone());
    //
    //             return Some(table.clone());
    //         }
    //     }
    //     None
    // }).expect(msg.as_str())
}

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