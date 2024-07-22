use proc_macro::TokenStream;

use enum_from::*;

mod enum_from;

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from_darling(input).into()
}
