extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(SingleFrom)]
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
        // This is a struct
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
        // This is a tuple
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
        // This is an enum
        syn::Body::Enum(ref variants) => {
            let cases = variants.iter().map(|variant| {
                // Check if the enum contains tuples only
                let source = match variant.data {
                    // Yes, this is an tuple
                    syn::VariantData::Tuple(ref tuple) => {
                        if tuple.len() != 1 {
                            panic!("This derive is defined for enum with variants as tuples with one field only!");
                        }
                        &tuple[0]
                    },
                    // Nope. This is struct or unit. We cannot handle these!
                    _ => panic!("This derive is defined for enums with tuples only, not structs or units!")
                };

                let ident = &variant.ident;
                quote!{
                    impl From<#source> for #name {
                        fn from(value: #source) -> #name {
                            #name::#ident(value)
                        }
                    }
                }
            });
            quote!{ #(#cases)* }
        },
        // Nope. This is a not struct, enum or tuple. We cannot handle these!
        _ => panic!("This derive is only defined for structs, enums and tuples, not for other!")
    }
}

//fn quote_from(name, sourceType, targetType) {
//    let ident = &variant.ident;
//    quote!{
//        impl From<#source> for #name {
//            fn from(value: #source) -> #name {
//                #name::#ident(value)
//            }
//        }
//    }
//}
