extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MacroProcedural)]
pub fn macro_procedural_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_macro_procedural(&ast)
}

fn impl_macro_procedural(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen_code = quote! {
        impl MacroProcedural for #name{
            fn greeting() {
                println!("Hello, My struct name is {}", stringify!(#name));
            }
        }
    };

    gen_code.into()
}
