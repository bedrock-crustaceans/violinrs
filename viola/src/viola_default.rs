use std::collections::HashMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, Expr, Token};
use syn::parse::{Parse, ParseStream};

pub struct ViolaDefaultDec {
    struct_ident: Ident,
    values: HashMap<Ident, Expr>
}

impl From<DeriveInput> for ViolaDefaultDec {
    fn from(value: DeriveInput) -> Self {
        let struct_ident = value.ident;
        let mut values: HashMap<Ident, Expr> = HashMap::new();

        for attr in value.attrs {
            if attr.path().is_ident("viola_default") {
                let arg = attr.parse_args::<Prop>().unwrap();

                values.insert(arg.ident, arg.value);
            }
        }

        Self {
            struct_ident,
            values
        }
    }
}

pub struct Prop {
    pub ident: Ident,
    pub value: Expr,
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        _ = input.parse::<Token![=]>()?;

        let expr = input.parse::<Expr>()?;

        Ok(Self {
            ident,
            value: expr
        })
    }
}

impl ToTokens for Prop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let value = &self.value;

        tokens.append_all(
            quote! {
                #ident: #value
            }
        )
    }
}

impl ToTokens for ViolaDefaultDec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.struct_ident;
        let values = self.values.clone().iter().map(
            |(ident, expr)| {
                Prop {
                    ident: ident.clone(),
                    value: expr.clone(),
                }
            }
        ).collect::<Vec<Prop>>();

        let maybe_comma = (values.len() > 0).then(|| {
            quote! {,}
        });

        tokens.append_all(quote! {
            impl ViolaDefault for #ident {
                fn viola_default() -> Self {
                    Self {
                        #(#values),*
                        #maybe_comma
                        ..Default::default()
                    }
                }
            }
        })
    }
}
