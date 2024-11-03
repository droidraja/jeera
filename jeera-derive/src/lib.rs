use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

#[proc_macro_derive(IsStarted)]
pub fn derive_is_started(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let Data::Enum(DataEnum { variants, .. }) = input.data else {
        panic!("is_started can only be used with enums");
    };

    let variant_matches = variants.iter().map(|v| &v.ident);
    let variant_matches_iter = variant_matches.clone();
    let variant_names = variants.iter().map(|v| &v.ident);
    let variant_names_iter = variant_names.clone();
    let expanded = quote! {
        impl IsStarted for #name {
            fn is_started(&self) -> bool {
                match self {
                    #(Self::#variant_matches_iter(lifecycle) => lifecycle.is_started(),)*
                }
            }

            fn is_finished(&self) -> bool {
                match self {
                    #(Self::#variant_matches(lifecycle) => lifecycle.is_finished(),)*
                }
            }

            fn is_failed(&self) -> bool {
                match self {
                    #(Self::#variant_names_iter(lifecycle) => lifecycle.is_finished(),)*
                }
            }

            fn name(&self) -> String {
                match self {
                    #(Self::#variant_names(_) => stringify!(#variant_names).to_string(),)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
