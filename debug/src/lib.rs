use std::fmt;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Expr, Ident};
use quote::{quote, ToTokens};


#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = &parse_macro_input!(input as DeriveInput);
    let s = &_input.ident;
    
    let mut fields: Vec<TokenStream2> = vec![];
    match &_input.data{
        syn::Data::Struct(ref data) => {
            match data.fields{
                syn::Fields::Named(ref f) => {
                    // field_names.append(
                    //     &mut f.named.iter().flat_map(
                    //         |f| &f.ident).collect::<Vec<&Ident>>());

                    for e in &f.named{
                        let mut debug_string = None;
                        for each in &e.attrs{
                            if each.path().is_ident("debug"){
                                let format = each.meta.require_name_value().unwrap();
                                // println!("{:?}", &format.value.to_token_stream().to_string());
                                debug_string = Some(&format.value);
                            }
                        }
                        fields.push(
                            field_representation(debug_string, e.ident.as_ref().unwrap())
                        )
                    }
                },
                _ => {

                }
            }
        },
        _ => {}
    }


    let extended = quote!{
        impl std::fmt::Debug for #s {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!(#s))
                    #( #fields)*
                .finish()
            }
        }
    };

    proc_macro::TokenStream::from(extended).into()
}



fn field_representation(debug_format: Option<&Expr>, ident: &Ident) -> TokenStream2{
    match debug_format{
        Some(format) => {
            quote!{
                .field(stringify!(#ident), format!(#format, &self.#ident))
            }
        },
        None => {
            quote!{
                .field(stringify!(#ident), &self.#ident)
            }
        }
    }
}