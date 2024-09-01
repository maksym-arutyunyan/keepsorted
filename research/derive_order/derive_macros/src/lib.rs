extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(First)]
pub fn derive_first(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as DeriveInput).ident;
    
    TokenStream::from(quote! {
        impl First for #name {
            fn is_first_implemented() -> bool {
                true
            }
        }
    })
}

#[proc_macro_derive(Second)]
pub fn derive_second(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as DeriveInput).ident;

    TokenStream::from(quote! {
        impl Second for #name {
            fn check_first_is_implemented() -> bool {
                let result = <#name as First>::is_first_implemented();
                println!(
                    "First is {}implemented, so Second {}!",
                    if result { "" } else { "NOT " },
                    if result { "works" } else { "fails" }
                );
                result
            }
        }
    })
}
