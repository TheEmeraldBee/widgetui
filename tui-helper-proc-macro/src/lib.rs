#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn};

/// Turns a function into a set.
/// Allows for simpler definitions of sets.
#[proc_macro_attribute]
pub fn set(_inner: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let ident = input.sig.ident.clone();
    let block = input.block.clone();
    let vis = input.vis.clone();

    let expanded = quote! {
        #vis struct #ident;

        impl Set for #ident {
            fn register_set(&self, app: App) -> App {
                (move |app: App| #block)(app)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(State)]
pub fn res(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();

    let expanded = quote! {
        impl State for #ident {}
    };

    TokenStream::from(expanded)
}
