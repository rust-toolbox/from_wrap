extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use quote::ToTokens;

#[proc_macro_derive(FromWrap, attributes(generate_from_wrap, not_generate_from_wrap))]
pub fn simple_from(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();
    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");
    let result = impl_simple_from(&ast);
    result.parse().expect("Couldn't parse string to tokens")
}

fn quote_struct(source: &ToTokens, target: &ToTokens, wrapper: &ToTokens, field: &ToTokens) -> Tokens {
    quote! {
        impl From<#source> for #target {
            fn from(value: #source) -> #target {
                #wrapper { #field: value }
            }
        }
    }
}

fn quote_tuple(source: &ToTokens, target: &ToTokens, wrapper: &ToTokens) -> Tokens {
    quote! {
        impl From<#source> for #target {
            fn from(value: #source) -> #target {
                #wrapper(value)
            }
        }
    }
}

fn impl_simple_from(ast: &syn::MacroInput) -> Tokens {
    let name = &ast.ident;
    // Check if the derive was specified for a struct, enum or tuple
    match ast.body {

        // This is a Struct
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            if fields.len() == 1 {
                let source = &fields[0].ty;
                let ident = &fields[0].ident;
                quote_struct(source, name, name, ident)
            } else {
                panic!("This derive is defined for struct with one field only!");
            }
        },

        // This is a Tuple
        syn::Body::Struct(syn::VariantData::Tuple(ref tuple)) => {
            if tuple.len() == 1 {
                let source = &tuple[0].ty;
                quote_tuple(source, name, name)
            } else {
                panic!("This derive is defined for tuple with one field only!");
            }
        },

        // This is an Enum
        syn::Body::Enum(ref variants) => {
            let mut accepted = Vec::<&syn::Variant>::new();

            // collect accepted variants if "generate_from_wrap" attribute exists
            for variant in variants {
                if variant.attrs.iter().find(|item| {
                    match item.value {
                        syn::MetaItem::Word(ref ident) => ident.as_ref() == "generate_from_wrap",
                        _ => false
                    }
                }).is_some() {
                    accepted.push(variant);
                }
            }

            // collect accepted variants if "generate_from_wrap" attribute not exists
            // (variants with attributes "not_generate_from_wrap" must are skipped)
            if accepted.len() == 0 {
                for variant in variants {
                    if !variant.attrs.iter().find(|item| {
                        match item.value {
                            syn::MetaItem::Word(ref ident) => ident.as_ref() == "not_generate_from_wrap",
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
                        if fields.len() == 1 {
                            let source = &fields[0].ty;
                            let field = &fields[0].ident;
                            quote_struct(source, name, &quote!(#name::#ident), field)
                        } else {
                            quote!()
                        }
                    },
                    // This is an Tuple
                    syn::VariantData::Tuple(ref tuple) => {
                        if tuple.len() == 1 {
                            let source = &tuple[0].ty;
                            quote_tuple(source, name, &quote!(#name::#ident))
                        } else {
                            quote!()
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