use proc_macro2::TokenStream;
use quote::quote;
use stylist_tailwind_core::components::classes;
use syn::parse2;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let a = parse2::<syn::LitStr>(input).unwrap();
    let mut css_classes: Vec<&str> = Vec::new();
    a.value().as_str().split(' ').into_iter().for_each(|class| {
        if let Some(css) = classes().get(class).cloned() {
            css_classes.push(css)
        } else {
            panic!("Class is not known");
        }
    });
    let final_css = css_classes.join(" ");
    quote! {
        #final_css
    }
}
