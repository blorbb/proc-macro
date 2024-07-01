use proc_macro2::TokenStream;

pub fn {{macro_name | snake_case}}_impl({% if macro_type == "Attribute" %}attr: TokenStream, {% endif %}input: TokenStream) -> TokenStream {
    todo!()
}
