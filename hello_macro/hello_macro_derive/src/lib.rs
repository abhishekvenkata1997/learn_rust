
extern crate proc_macro; //crate comes with rust
//proc_macro crate allows to read and manipulate rust code

use proc_macro::TokenStream; 
use quote::quote; //quote converts this data structure back into code
use syn; //syntax -> Allows to take string of rust code, convert into a syntax tree data structure to operate on

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    //construct a representation of rust code as a syntax tree
    // that we can manipulate

    let ast = syn::parse(input).unwrap(); //parse token stream into abstract syntax tree

    //build trait implementation
    impl_hello_macro(&ast) //transform the syntax tree
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    
    let name = &ast.ident;
    let gen = quote! {
        //#name will be replaced by the name of our type
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, macro my name is {}!",
                stringify!(#name));
            }
        }
    };
    gen.into()
}