use proc_macro::TokenStream;
use std::str::FromStr;

use heck::SnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, Lit, Meta, NestedMeta, punctuated::Punctuated};

pub fn derive(ast: syn::DeriveInput) -> TokenStream {
    let table = parse_attrs(&ast);
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

    let add_arguments_str = fields_vec.iter().map(|s|
        format!(r#".add(&self.{}) "#, s)
    ).collect::<String>();

    let from_row_str = fields_vec.iter().map(|s|
        format!(r#"{0}: row.decode("{0}")?, "#, s)
    ).collect::<String>();

    let token = quote!(
        impl cherry::Cherry for #ident {
            type Database = cherry::mysql::MySql; // todo the other database.

            fn table() -> &'static str {
                #table
            }
            fn columns() -> Vec<&'static str> {
                vec![ [fields_str] ]
            }

            fn arguments<'a>(&'a self, arguments: &mut cherry::Arguments<'a, Self::Database>) {
                arguments [add_arguments_str] ;
            }

            fn from_row(row: &cherry::Row<Self::Database>) -> Result<Self, cherry::error::Error> {
                Ok( Self { [from_row_str] } )
            }
        }
    );

    let token = token.to_string()
        .replace("[fields_str]", fields_str.as_str())
        .replace("[from_row_str]", from_row_str.as_str())
        .replace("[add_arguments_str]", add_arguments_str.as_str());

    // println!("{}", token);

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