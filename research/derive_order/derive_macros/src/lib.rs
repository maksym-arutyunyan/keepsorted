extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(First)]
pub fn derive_first(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    
    let expanded = quote! {
        impl First for #name {
            fn is_first_implemented() -> bool {
                true
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Second)]
pub fn derive_second(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl Second for #name {
            fn check_first_is_implemented() -> bool {
                if <#name as First>::is_first_implemented() {
                    println!("First is implemented, so Second works!");
                    true
                } else {
                    println!("First is NOT implemented, so Second fails!");
                    false
                }
            }
        }
    };

    TokenStream::from(expanded)
}
