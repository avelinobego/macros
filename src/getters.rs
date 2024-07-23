use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub(crate) fn impl_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Generate the implementation of the Default trait
    let gen = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => {
                let getters = fields_named.named.iter().map(|f| {
                    let f_name = f.ident.as_ref().unwrap();
                    let f_type = &f.ty;
                    quote! {
                       pub fn #f_name(&self) -> &#f_type {
                           &self.#f_name
                       }
                    }
                });

                quote! {
                    impl #name {
                        #(#getters)*
                    }
                }
            }
            _ => panic!("Getters can only be used with structs with named fields"),
        },
        _ => panic!("Setters can only be used with structs"),
    };

    // Convert the generated code into a TokenStream and return it
    gen.into()
}
