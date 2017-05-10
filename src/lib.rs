extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(SingleFrom, attributes(generate_from, not_generate_from))]
pub fn single_from(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();
    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let result = impl_single_from(&ast);
    result.parse().expect("Couldn't parse string to tokens")
}

fn impl_single_from(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    // Check if the derive was specified for a struct, enum or tuple
    match ast.body {
        // This is a Struct
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            if fields.len() != 1 {
                panic!("This derive is defined for struct with one field only!");
            }
            let source = &fields[0].ty;
            let ident = &fields[0].ident;
            quote!{
                impl From<#source> for #name {
                    fn from(value: #source) -> #name {
                        #name { #ident: value }
                    }
                }
            }
        },
        // This is a Tuple
        syn::Body::Struct(syn::VariantData::Tuple(ref tuple)) => {
            if tuple.len() != 1 {
                panic!("This derive is defined for tuple with one field only!");
            }
            let source = &tuple[0].ty;
            quote!{
                impl From<#source> for #name {
                    fn from(value: #source) -> #name {
                        #name( value )
                    }
                }
            }
        },
        // This is an Enum
        syn::Body::Enum(ref variants) => {
            let mut accepted = Vec::<&syn::Variant>::new();

            // collect accepted variants if "generate_from" attribute exists
            for variant in variants {
                if variant.attrs.iter().find(|item| {
                    match item.value {
                        syn::MetaItem::Word(ref ident) => ident.as_ref() == "generate_from",
                        _ => false
                    }
                }).is_some() {
                    accepted.push(variant);
                }
            }

            // collect accepted variants if "generate_from" attribute not exists
            // (variants with attributes "not_generate_from" must are skipped)
            if accepted.len() == 0 {
                for variant in variants {
                    if !variant.attrs.iter().find(|item| {
                        match item.value {
                            syn::MetaItem::Word(ref ident) => ident.as_ref() == "not_generate_from",
                            _ => false
                        }
                    }).is_some() {
                        accepted.push(variant);
                    }
                }
            }

            // Produce quoted cases for accepted variants
            let cases = accepted.iter().map(|variant| {
                let ident = &variant.ident;
                // Get enum variant type only for Struct or Tuple
                match variant.data {
                    // This is a Struct
                    syn::VariantData::Struct(ref fields) => {
                        if fields.len() != 1 {
                            panic!("This derive is defined for enum with variants as structs or tuples with one field only!");
                        }
                        let source = &fields[0].ty;
                        let field = &fields[0].ident;
                        quote!{
                            impl From<#source> for #name {
                                fn from(value: #source) -> #name {
                                    #name::#ident { #field: value }
                                }
                            }
                        }
                    },
                    // This is an Tuple
                    syn::VariantData::Tuple(ref tuple) => {
                        if tuple.len() != 1 {
                            panic!("This derive is defined for enum with variants as structs or tuples with one field only!");
                        }
                        let source = &tuple[0].ty;
                        quote!{
                            impl From<#source> for #name {
                                fn from(value: #source) -> #name {
                                    #name::#ident(value)
                                }
                            }
                        }
                    },
                    // Nope. This is an Unit.
                    syn::VariantData::Unit => quote!()
                } // return quoted
            });
            quote!{ #(#cases)* }
        },
        // Nope. This is a not Struct, Enum or Tuple. We cannot handle these!
        _ => panic!("This derive is only defined for struct, enum or tuple, not for other!")
    }
}