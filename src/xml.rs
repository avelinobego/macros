use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, FieldsNamed};

pub(crate) fn impl_xml(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let gen = match input.data {
        syn::Data::Struct(d_struct) => match d_struct.fields {
            Fields::Named(ref fields) => {
                let gen_code = create_itens(fields);

                quote! {
                    impl std::fmt::Display for #struct_name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            let mut buffer = String::new();
                            buffer.push_str(&format!("<{0}>",stringify!(#struct_name)));
                            #(#gen_code)*
                            buffer.push_str(&format!("</{0}>",stringify!(#struct_name)));
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

fn create_itens(fields: &FieldsNamed) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let result = fields.named.iter().map(|f| {
        let f_name = f.ident.as_ref().unwrap();
        quote! {
            //TODO: Criar função, atributos para nomes customizados, attrib e namespaces.
            buffer.push_str(&format!("<{0}>{1}</{0}>",stringify!(#f_name), self.#f_name));
        }
    });
    result.clone()
}
