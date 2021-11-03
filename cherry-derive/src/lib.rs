use proc_macro::TokenStream;

pub(crate) mod derive_cherry;

#[proc_macro_derive(Cherry, attributes(cherry))]
pub fn derive_cherry(input: TokenStream) -> TokenStream {
    derive_cherry::derive(syn::parse_macro_input!(input))
}
