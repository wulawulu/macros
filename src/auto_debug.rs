use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDebug only supports structs");
    };
    let fields = fields.iter().map(|f| {
        if f.skip {
            quote! {}
        } else {
            let ident = f.ident.as_ref().unwrap();
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
    impl #generics ::core::fmt::Debug for #ident #generics {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct(stringify!(#ident))
                #(#fields)*
                .finish()
                }
        }
    }
}
