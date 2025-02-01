pub(crate) mod viola_declaration;
mod viola_default;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::viola_declaration::ViolaDeclaration;
use crate::viola_default::ViolaDefaultDec;

#[proc_macro]
pub fn viola(input: TokenStream) -> TokenStream {
    let declaration = parse_macro_input!(input as ViolaDeclaration);

    (quote! {
        #declaration
    }).into()
}

#[proc_macro_derive(
   ViolaDefault,
   attributes(viola_default)
)]
pub fn viola_default(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    
    let viola_default_trait_dec = ViolaDefaultDec::from(derive);

    (quote! {
        #viola_default_trait_dec
    }).into()
}