#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn};

#[proc_macro_derive(FromState)]
pub fn from_state(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl FromStates for #ident {
            fn from_state(states: &mut States) -> Result<State<Self>, Box<dyn std::error::Error>> {
                states.get::<Self>()
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

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

// // Function Count,
// // Example impl_tuple_set!(3)

// #[proc_macro]
// pub fn impl_tuple_set(input: TokenStream) -> TokenStream {
//     use syn::parse::Parser;
//     let data = syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated
//         .parse2(input.into())
//         .unwrap();
// }
