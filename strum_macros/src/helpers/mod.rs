pub use self::case_style::snakify;
pub use self::inner_variant_props::HasInnerVariantProperties;
pub use self::type_props::HasTypeProperties;
pub use self::variant_props::HasStrumVariantProperties;

pub mod case_style;
pub mod inner_variant_props;
mod metadata;
pub mod type_props;
pub mod variant_props;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn non_unit_variant_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "This macro only supports enums of strictly unit variants. Consider \
        using it in conjunction with [`EnumDiscriminants`]",
    )
}

pub fn strum_discriminants_passthrough_error(span: &impl Spanned) -> syn::Error {
    syn::Error::new(
        span.span(),
        "expected a pass-through attribute, e.g. #[strum_discriminants(serde(rename = \"var0\"))]",
    )
}

pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("Found multiple occurrences of strum({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first one here"));
    e
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    Str(syn::LitStr),
    Num(syn::LitInt),
    Bool(syn::LitBool),
}

impl syn::parse::Parse for PropertyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = if input.peek(syn::LitBool) {
            PropertyValue::Bool(input.parse()?)
        } else if input.peek(syn::LitInt) {
            PropertyValue::Num(input.parse()?)
        } else {
            PropertyValue::Str(input.parse()?)
        };
        Ok(value)
    }
}

impl quote::ToTokens for PropertyValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            PropertyValue::Str(s) => s.to_tokens(tokens),
            PropertyValue::Num(n) => n.to_tokens(tokens),
            PropertyValue::Bool(b) => b.to_tokens(tokens),
        }
    }
}
