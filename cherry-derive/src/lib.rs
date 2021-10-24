use proc_macro::TokenStream;

pub(crate) mod impl_cherry;

#[proc_macro_derive(Cherry, attributes(cherry))]
pub fn derive_cherry(input: TokenStream) -> TokenStream {
    impl_cherry::derive(syn::parse_macro_input!(input))
}
