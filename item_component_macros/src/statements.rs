use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::Into;
use syn::parse::{Parse, ParseStream};
use syn::{Token, Type};

mod keywords {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(has);
    custom_keyword!(with);
}

pub(crate) struct ComponentData {
    pub name: String,
    pub property_id: String,
    pub props: Vec<PropertyDeclaration>,
    pub modifiers: Vec<Modifier>
}

fn string_from_literal(lit: Literal) -> String {
    lit.to_string().strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string()
}

// pub(crate) fn parse_statements(tokens: VecDeque<Token>) -> ComponentData {
//     let mut tks = tokens.clone();
//     dbg!(tks.clone());
//
//     let (name, property_id) = parse_name(&mut tks);
//
//     let props = parse_prop_declarations(&mut tks);
//     // let props = Vec::new();
//
//     ComponentData {
//         props,
//         name,
//         property_id
//     }
// }
//
// fn parse_name(tks: &mut VecDeque<Token>) -> (String, String) {
//     let name_tk = tks.pop_front().unwrap_or(Token::End);
//     dbg!(&name_tk);
//     if name_tk != Token::Identifier("name".to_string()) {
//         panic!("First should always be 'name'");
//     }
//
//     let arrow_tk = tks.pop_front().unwrap_or(Token::End);
//     if arrow_tk != Token::Operator(OperatorType::Equal) {
//         panic!("After 'name' always should be '=");
//     }
//
//     let ident_tk = tks.pop_front().unwrap_or(Token::End);
//
//     let name = if let Token::Identifier(name) = ident_tk {
//         name
//     } else {
//         panic!("A name should follow after '='")
//     };
//
//     let for_kw = tks.pop_front().unwrap_or(Token::End);
//
//     if for_kw != Token::Keyword(KeywordType::For) {
//         panic!("After the name should always follow 'for'")
//     };
//
//     let id_ident = tks.pop_front().unwrap_or(Token::End);
//
//     let id = if let Token::String(id) = id_ident {
//         tks.pop_front(); // ;
//         id
//     } else {
//         panic!("After the 'for' should always follow a property name")
//     };
//
//     (name, id)
// }

// fn parse_prop_declarations(tokens: &mut VecDeque<Token>) -> Vec<PropertyDeclaration> {
//     let mut props: Vec<PropertyDeclaration> = Vec::new();
//
//     while tokens.front().cloned().unwrap_or(Token::End) != Token::End {
//         props.push(parse_unique_statement(tokens));
//         dbg!(tokens.front());
//     }
//
//     props
// }

// fn parse_unique_statement(tokens: &mut VecDeque<Token>) -> PropertyDeclaration {
//     let name: String = if let Token::Identifier(name) = tokens.pop_front().unwrap_or(Token::End) {
//         name
//     } else {
//         panic!("Name should always come first");
//     };
//     if tokens.pop_front().unwrap_or(Token::End) != Token::Keyword(KeywordType::Has) {
//         panic!("After name should be the keyword 'has'")
//     }
//     let data_type: String = if let Token::Identifier(data_type) = tokens.pop_front().unwrap_or(Token::End) {
//         data_type
//     } else {
//         panic!("After name should come the data type")
//     };
//     let mut buf = tokens.pop_front().unwrap_or(Token::End);
//     if buf != Token::Keyword(KeywordType::For) {
//         panic!("After the data type should follow the keyword 'for', but found {:?}", buf)
//     };
//     let property_id: String = if let Token::String(property_id) = tokens.pop_front().unwrap_or(Token::End) {
//         property_id
//     } else {
//         panic!("After the data type should come the property id")
//     };
//     tokens.pop_front().unwrap_or(Token::End); // ;
//
//
//     PropertyDeclaration {
//         name,
//         property_id,
//         data_type
//     }
// }

#[derive(Clone)]
pub(crate) struct PropertyDeclaration {
    name: Ident,
    data_type: Type,
    property_id: String,
    modifiers: Vec<Modifier>
}

impl ToTokens for PropertyDeclaration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.clone();
        let data_type = self.data_type.clone();
        let property_id = self.property_id.clone();

        let pub_or_not = if self.modifiers.contains(&Modifier::Public) {
            quote! {
                pub
            }
        } else {
            quote! {}
        };

        tokens.append_all(quote! {
            #[serde(rename(serialize = #property_id))]
            #pub_or_not #name: #data_type,
        });
    }
}

impl Parse for ComponentData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<keywords::name>()?; // name
        _ = input.parse::<Token![=]>()?; // equals sign
        let ident = Ident::parse(input)?; // the name itself
        dbg!(&ident);
        _ = input.parse::<Token![for]>()?; // for
        let param_id = Literal::parse(input)?;
        dbg!(&param_id);

        let mut has_modifiers = true;
        let mut modifiers: Vec<Modifier> = Vec::new();

        if input.peek(Token![;]) {
            _ = input.parse::<Token![;]>()?;
            has_modifiers = false;
        } else {
            _ = input.parse::<keywords::with>()?;
        }

        if has_modifiers {
            modifiers = parse_zero_or_more(input);
            _ = input.parse::<Token![;]>()?;
        }

        let props: Vec<PropertyDeclaration> = parse_zero_or_more(input);

        Ok(ComponentData {
            name: ident.to_string(),
            property_id: string_from_literal(param_id),
            props,
            modifiers
        })
    }
}

impl ToTokens for ComponentData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let prop_id = self.property_id.clone();
        let name = Ident::new(&format!("Item{}Component", &self.name), Span::call_site());
        let props = self.props.iter().clone();

        let impl_stmt = quote! {
            impl ItemComponent for #name {
                fn serialize(&self) -> String {
                    let value = format!("\"{}\": {}", #prop_id, serde_json::to_string_pretty(&self).unwrap());

                    value
                }
            }

            impl Buildable for #name {}
        };

        let arg_vals: Vec<Ident> = props.clone().map(|x| x.name.clone()).collect();
        let arg_types: Vec<Type> = props.clone().map(|x| x.data_type.clone()).collect();
        let arg_modifiers: Vec<Vec<Modifier>> = props.clone().map(|x| x.modifiers.clone()).collect();

        let new_fn = NewFun {
            types: arg_types,
            names: arg_vals,
            modifiers: arg_modifiers
        };

        let fun = quote! {
            impl #name {
                #new_fn
            }
        };

        let using_stmt_fns = get_using_statements_for(self.props.clone());

        let using_stmt = quote! {
            impl #name {
                #using_stmt_fns
            }
        };

        let additional_derives = get_additional_derives(&self);
        

        let tks: TokenStream = quote! {
            #[derive(serde::Serialize, Debug, Clone)]
            #[serde(rename = #prop_id)]
            #additional_derives
            pub struct #name {
                #(#props)*
            }

            #impl_stmt

            #fun

            #using_stmt
        };

        tokens.append_all(tks);
    }
}

fn parse_zero_or_more<T>(input: ParseStream) -> Vec<T>
where
    T: Parse {
    let mut result_vec = Vec::new();

    while let Ok(item) = input.parse() {
        result_vec.push(item);
    }

    result_vec
}

impl Parse for PropertyDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id = Ident::parse(input)?;
        _ = input.parse::<keywords::has>()?;
        let data_type = Type::parse(input)?;
        _ = input.parse::<Token![for]>()?;
        let param_id = Literal::parse(input)?;
        dbg!(&param_id);

        let mut has_modifiers = true;

        if input.peek(Token![;]) {
            has_modifiers = false;
        } else {
            _ = input.parse::<keywords::with>();
        }

        let mut modifiers = Vec::new();

        if has_modifiers {
            modifiers = parse_zero_or_more(input);
        }

        _ = input.parse::<Token![;]>()?;

        Ok(PropertyDeclaration {
            property_id: string_from_literal(param_id),
            data_type,
            name: id,
            modifiers
        })
    }
}

struct NewFun {
    names: Vec<Ident>,
    types: Vec<Type>,
    modifiers: Vec<Vec<Modifier>>,
}

impl ToTokens for NewFun {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut arg_arr = Vec::new();

        for idx in 0..self.names.len() {
            let arg = &self.names[idx];
            let tp = &self.types[idx];
            let modifiers = &self.modifiers[idx];

            arg_arr.push(NewFunArgPair {
                name: arg.clone(),
                data_type: tp.clone(),
                modifiers: modifiers.clone()
            })
        }

        let mut set_arr = Vec::new();

        for idx in 0..self.names.len() {
            let arg = &self.names[idx];
            let tp = &self.types[idx];
            let modifiers = &self.modifiers[idx];

            set_arr.push(NewFunSetPair {
                name: arg.clone(),
                data_type: tp.clone(),
                modifiers: modifiers.clone()
            })
        }

        let tks: TokenStream = quote! {
            pub fn new(#(#arg_arr),*) -> Self {
                Self {
                    #(#set_arr),*
                }
            }
        };

        tks.to_tokens(tokens);
    }
}

struct NewFunArgPair {
    name: Ident,
    data_type: Type,
    modifiers: Vec<Modifier>
}

struct NewFunSetPair {
    name: Ident,
    data_type: Type,
    modifiers: Vec<Modifier>
}

impl ToTokens for NewFunArgPair {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name= self.name.clone();
        let data_type = self.data_type.clone();

        let tks: TokenStream = if self.modifiers.contains(&Modifier::IntoModifier) {
            quote! {
                #name: impl Into<#data_type>
            }
        } else {
            quote! {
                #name: #data_type
            }
        }.into();

        tokens.append_all(
            tks
        );
    }
}

impl ToTokens for NewFunSetPair {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.clone();

        let tks: TokenStream = if self.modifiers.contains(&Modifier::IntoModifier) {
            quote! {
                #name: #name.into()
            }
        } else {
            quote! {
                #name: #name
            }
        }.into();

        tks.to_tokens(tokens);
    }
}

#[derive(Clone, PartialEq)]
enum Modifier {
    IntoModifier,
    Undetermined,
    Public,
    Transparency,
    UsingFn
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw = Literal::parse(input)?;

        let kw = string_from_literal(kw);

        // if kw == "into".to_string() {
        //     Ok(IntoModifier)
        // } else if kw == "public".to_string() {
        //     Ok(Public)
        // } else {
        //     Ok(Undetermined)
        // }

        match kw.as_str() {
            "into" => Ok(Modifier::IntoModifier),
            "public" => Ok(Modifier::Public),
            "using" => Ok(Modifier::UsingFn),
            "transparency" => Ok(Modifier::Transparency),
            _ => Ok(Modifier::Undetermined)
        }
    }
}

fn get_using_statements_for(props: Vec<PropertyDeclaration>) -> TokenStream {
    let mut tks = quote! {};

    for prop in props.iter() {
        if prop.modifiers.contains(&Modifier::UsingFn) {
            let prop_name = prop.name.clone();
            let prop_type = prop.data_type.clone();
            let fn_ident = Ident::new(
                &format!("using_{}", prop_name.to_string()),
                prop_name.span()
            );

            tks.append_all(
                if prop.modifiers.contains(&Modifier::IntoModifier) {
                    quote! {
                        pub fn #fn_ident(&mut self, #prop_name: impl Into<#prop_type>) -> Self {
                            let mut sc = self.clone();

                            sc.#prop_name = #prop_name.into();

                            sc
                        }
                    }
                } else {
                    quote! {
                        pub fn #fn_ident(&mut self, #prop_name: #prop_type) -> Self {
                            let mut sc = self.clone();

                            sc.#prop_name = #prop_name;

                            sc
                        }
                    }
                }
            );
        }
    }

    tks
}

fn get_additional_derives(component_data: &ComponentData) -> TokenStream {
    let mut tks = quote! {};

    if component_data.modifiers.contains(&Modifier::Transparency) {
        tks.append_all(quote! {
            #[serde(transparent)]
        })
    }

    tks
}