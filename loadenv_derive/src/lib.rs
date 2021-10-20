use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, NestedMeta, parse_macro_input};

#[proc_macro_derive(LoadEnv, attributes(env))]
pub fn build_derive(ts: TokenStream) -> TokenStream {
    let out = parse_macro_input!(ts as DeriveInput);

    let mut load_tokens = vec![
        quote! {}
    ];

    
    match out.data {
        syn::Data::Struct(data) => {
            data.fields.into_iter().for_each(|f| {

                let field_ident = f.ident.unwrap();
            
                for attr in f.attrs {
                    if attr.path.is_ident("env") {
                        match attr.parse_meta() {
                            Ok(meta) => {
                                match meta {
                                    syn::Meta::List(l) => {
                                        let tokens: Vec<NestedMeta> = l.nested.into_iter().collect();
                                        match tokens.len() {
                                            1 => {
                                                let envname = tokens.get(0).unwrap();
                                                load_tokens.push(quote! {
                                                    
                                                        match std::env::var(#envname) {
                                                            Ok(value) => {
                                                                self.#field_ident = loadenv::EnvValue::new(value).into();
                                                            },
                                                            Err(err) => {}
                                                        }
                                                    
                                                });
                                            }
                                            2 => {

                                                
                                                
                                                let envname = tokens.get(0).unwrap();
                                                let default = tokens.get(1).unwrap();


                                                load_tokens.push(quote! {
                                                    
                                                        self.#field_ident = #default.into();
                                                        match std::env::var(#envname) {
                                                            Ok(value) => {
                                                                self.#field_ident = loadenv::EnvValue::new(value).into();
                                                            },
                                                            Err(err) => {}
                                                        }
                                                    
                                                });
                                            }
                                            _ => {
        
                                            }
                                        }
                                        
                                    },
                                    _ => {
                                        panic!("invalid env attribute paramaters");
                                    }
                                }
                                
                            },
                            Err(err) => {
                                println!("EEEEE {:?}", err)
                            }
                        }
                        break;
                    }
                }
            });
        },
        _ => panic!("not support Union & Enum"),
    }
    
    let struct_name = out.ident;
    let updates = load_tokens.into_iter().fold(quote! {}, |acc, new| quote! {#acc #new});

    let expanded = quote! {
        impl #struct_name {
            pub fn load_env(&mut self) {
                #updates;
            }
        }
    };
    
    expanded.into()
}