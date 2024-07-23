use proc_macro::TokenStream;

mod setters;
mod getters;
mod xml;

#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    getters::impl_getters(input)
}

#[proc_macro_derive(Setters)]
pub fn setters(input: TokenStream) -> TokenStream {
    setters::impl_setters(input)
}

#[proc_macro_derive(XML)]
pub fn xml_macro(input: TokenStream) -> TokenStream {
    xml::impl_xml(input)
}