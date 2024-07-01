use proc_macro::TokenStream;

{%- if include_error %}
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
{% else %}

{% endif -%}

{% case macro_type -%}
    {%- when "Function-like" -%}

#[proc_macro]
pub fn {{macro_name}}(input: TokenStream) -> TokenStream {
    {{crate_name}}_core::{{macro_name | snake_case}}_impl(input.into()).into()
}

    {%- when "Derive" -%}

#[proc_macro_derive({{macro_name}})]
pub fn {{macro_name | snake_case}}(input: TokenStream) -> TokenStream {
    {{crate_name}}_core::{{macro_name | snake_case}}_impl(input.into()).into()
}

    {%- when "Attribute" -%}

#[proc_macro_attribute]
pub fn {{macro_name}}(attr: TokenStream, input: TokenStream) -> TokenStream {
    {{crate_name}}_core::{{macro_name | snake_case}}_impl(attr.into(), input.into()).into()
}

{%- endcase %}
