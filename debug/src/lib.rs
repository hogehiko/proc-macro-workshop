use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;


#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = &parse_macro_input!(input as DeriveInput);
    let s = &_input.ident;
    
    let mut field_names: Vec<&Ident> = vec![];
    match &_input.data{
        syn::Data::Struct(ref data) => {
            match data.fields{
                syn::Fields::Named(ref f) => {
                    field_names.append(
                        &mut f.named.iter().flat_map(
                            |f| &f.ident).collect::<Vec<&Ident>>());
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
                    #(. field(stringify!(#field_names), &self.#field_names))*
                .finish()
            }
        }
    };

    proc_macro::TokenStream::from(extended).into()
}
