mod tw;

#[proc_macro]
pub fn tw(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tw::macro_fn(input.into()).into()
}
