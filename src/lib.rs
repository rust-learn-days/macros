use proc_macro::TokenStream;

use auto_deref::*;
use enum_from::*;

mod auto_deref;
mod enum_from;

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from_darling(input).into()
}

#[proc_macro_derive(DerefInfo, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_auto_deref(input).into()
}
