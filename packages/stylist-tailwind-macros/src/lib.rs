extern crate proc_macro;
use proc_macro::TokenStream;
use stylist::{css, Style, StyleSource};

pub mod components;

#[proc_macro]
pub fn text_xs(_item: TokenStream) -> TokenStream {
    TokenStream::new(Style::new(css!("font-size: 0.75rem; line-height: 1rem;")).unwrap())
}
