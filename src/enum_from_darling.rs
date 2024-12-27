use darling::ast::{Data, Fields, Style};
use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariant, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariant {
    ident: syn::Ident,
    fields: Fields<EnumVariantField>,
}

#[derive(Debug, FromField)]
struct EnumVariantField {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };
    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            Self::#var(value)
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
