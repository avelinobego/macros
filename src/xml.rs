use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

pub(crate) fn impl_xml(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = match input.data {
        syn::Data::Struct(d_struct) => match d_struct.fields {
            Fields::Named(ref fields) => {
                let gen_code = fields.named.iter().map(|f| {
                    let f_name = f.ident.as_ref().unwrap();
                    quote! {
                        buffer.push_str(&format!("<{0}>{1}</{0}>",stringify!(#f_name), self.#f_name));
                    }
                });

                quote! {
                    impl std::fmt::Display for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            let mut buffer = String::new();
                            #(#gen_code)*
                            write!(f, "{}", buffer)
                        }
                    }
                }
            }
            _ => panic!("XML can only be used with structs with named fields"),
        },
        _ => panic!("XML can only be used with structs"),
    };

    r#gen.into()
}
