use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumFromVariant, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumFromVariant {
    ident: syn::Ident,
    fields: Fields<EnumFromFields>,
}

#[derive(Debug, FromField)]
struct EnumFromFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("failed to parse input")
    else {
        panic!("failed to parse input")
    };
    let from_impls = data.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.iter().count() == 1 => {
                let field = variant.fields.iter().next().expect("no fields");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            #ident::#variant_ident(value)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
}
