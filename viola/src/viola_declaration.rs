use std::fmt::Pointer;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{braced, bracketed, Expr, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod punct {
    use syn::{custom_keyword, custom_punctuation};

    custom_punctuation!(DollarInto, $>);
    custom_keyword!(vector);
    custom_keyword!(via);
}

pub struct Declaration {
    pub ident: Ident,
    pub should_build: bool,
}

pub(crate) enum ViolaDeclaration {
    Struct(Declaration, Punctuated<Property, Token![,]>),
    Vector(Vec<ViolaDeclaration>),
    ViaStmt(Declaration, Ident)
}

pub struct Prop {
    pub ident: Ident,
    pub wrap_in_into: bool,
    pub wrap_in_some: bool
}

enum Property {
    Expr(Prop, Expr),
    Viola(Prop, ViolaDeclaration),
}

impl Parse for ViolaDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(punct::vector) {
            input.parse::<punct::vector>()?;
            
            let content;
            bracketed!(content in input);
            
            let pnct: Punctuated<ViolaDeclaration, Token![,]> = content.parse_terminated(ViolaDeclaration::parse, Token![,])?;
            
            Ok(Self::Vector(
                pnct.into_iter().collect(),
            ))
        } else if input.peek2(punct::via) {
            let mut should_build = false;
            
            let struct_ident = input.parse::<Ident>()?;
            
            input.parse::<punct::via>()?;
            
            let fn_ident = input.parse::<Ident>()?;
            
            if input.peek(Token![!]) {
                input.parse::<Token![!]>()?;
                should_build = true;
            }
            
            Ok(Self::ViaStmt(
                Declaration {
                    ident: struct_ident,
                    should_build,
                },
                fn_ident
            ))
        } else {
            if input.peek(Token![@]) {
                input.parse::<Token![@]>()?;
            }

            let struct_ident = input.parse::<Ident>()?;

            let content;

            braced!(content in input);

            let props: Punctuated<Property, Token![,]> = content.parse_terminated(Property::parse, Token![,])?;

            let mut should_build = false;

            if input.peek(Token![!]) {
                should_build = true;
                input.parse::<Token![!]>()?;
            }

            Ok(Self::Struct(Declaration {
                ident: struct_ident,
                should_build,
            }, props,))
        }
    }
}
//
// impl Parse for Prop {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let ident = input.parse::<Ident>()?;
//
//         _ = input.parse::<Token![=]>()?;
//
//         let mut wrap_in_into = false;
//         let mut wrap_in_viola = false;
//
//         if input.peek(Token![$]) {
//             input.parse::<Token![$]>()?;
//             wrap_in_into = true;
//         }
//
//         if input.peek(Token![@]) {
//             input.parse::<Token![@]>()?;
//             wrap_in_viola = true;
//         }
//
//         let expr = input.parse::<>()?;
//
//         Ok(Self {
//             ident,
//             wrap_in_into
//         })
//     }
// }

impl ToTokens for ViolaDeclaration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            ViolaDeclaration::Struct(decl, props) => {
                let ident = &decl.ident;
                let props = &props.iter().clone().collect::<Vec<&Property>>();
                let should_build = &decl.should_build;

                let build_call = should_build.then(|| {
                    quote! {
                        .build()
                    }
                });

                tokens.append_all(quote! {
                    #ident {
                        #(#props),*,
                        ..ViolaDefault::viola_default()
                    }
                    #build_call
                })
            }
            ViolaDeclaration::Vector(elements) => {
                tokens.append_all(quote! {
                    vec![
                        #(#elements),*
                    ]
                }) 
            },
            ViolaDeclaration::ViaStmt(decl, fn_ident) => {
                let struct_ident = &decl.ident;
                
                let maybe_build = decl.should_build.then(|| {
                    quote! { .build() }
                });
                
                tokens.append_all(quote! {
                    #struct_ident::#fn_ident()#maybe_build
                })
            }
        }
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (ident, value) = get_prop_val(&self);

        tokens.append_all(
            quote! {
                #ident: #value
            }
        )
    }
}

fn get_prop_val(prop: &Property) -> (&Ident, TokenStream) {
    if let Property::Viola(prop, value) = prop {
        (&prop.ident, wrap_in_some(prop, quote! { #value }))
    } else if let Property::Expr(prop, value) = prop {
        (&prop.ident, wrap_in_some(prop, quote! { #value }))
    } else {
        unreachable!()
    }
}

fn wrap_in_some(prop: &Prop, value: TokenStream) -> TokenStream {
    let value = wrap_in_into(prop, value);
    if prop.wrap_in_some {
        quote! {
            Some(#value)
        }
    } else {
        quote! {
            #value
        }
    }
}

fn wrap_in_into(prop: &Prop, value: TokenStream) -> TokenStream {
    if prop.wrap_in_into {
        quote! {
            (#value).into()
        }
    } else {
        quote! {
            #value
        }
    }
}

impl Parse for Property {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        _ = input.parse::<Token![=]>()?;

        let mut wrap_in_into = false;
        let mut wrap_in_some = false;

        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            wrap_in_some = true;
        }

        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            wrap_in_into = true;
        }

        let prop = Prop {
            ident,
            wrap_in_into,
            wrap_in_some,
        };

        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;

            let viola = input.parse::<ViolaDeclaration>()?;

            Ok(Property::Viola(prop, viola))
        } else {
            let expr = input.parse::<Expr>()?;

            Ok(Property::Expr(prop, expr))
        }
    }
}