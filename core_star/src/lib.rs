extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Component)]
pub fn impl_component(stream: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(stream as DeriveInput);
    let struct_name = &ast.ident;

    quote!(
        impl Component for #struct_name {}
    ).into()
}
