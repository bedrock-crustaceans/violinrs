mod statements;

use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse_macro_input;
use crate::statements::{ComponentData};

fn get_source(call_site: Option<String>) -> String {
    let text = call_site.unwrap_or("".to_string());
    dbg!(&text);

    text.trim_start_matches("item_component!").trim_start_matches(" ").trim_start_matches("{").trim_end_matches("}").to_string()
}

#[proc_macro]
pub fn item_component(input: TokenStream) -> TokenStream {
    let span = Span::call_site();
    dbg!("Parse start");

    let res = parse_macro_input!(input as ComponentData);

    (quote! { #res }).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
