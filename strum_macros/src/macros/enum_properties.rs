use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::PropertyValue;
use crate::helpers::{non_enum_error, HasStrumVariantProperties, HasTypeProperties};

pub fn enum_properties_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };
    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    let mut string_props = Vec::new();
    let mut num_props = Vec::new();
    let mut bool_props = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;
        let mut string_arms = Vec::new();
        let mut bool_arms = Vec::new();
        let mut num_arms = Vec::new();
        // But you can disable the messages.
        if variant_properties.disabled.is_some() {
            continue;
        }

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        for (key, value) in variant_properties.string_props {
            match value {
                PropertyValue::Str(s) => {
                    string_arms.push(quote! { #key => ::core::option::Option::Some( #s )})
                }
                PropertyValue::Num(n) => {
                    num_arms.push(quote! { #key => ::core::option::Option::Some( #n )})
                }
                PropertyValue::Bool(b) => {
                    bool_arms.push(quote! { #key => ::core::option::Option::Some( #b )})
                }
            }
        }

        string_arms.push(quote! { _ => ::core::option::Option::None });
        bool_arms.push(quote! { _ => ::core::option::Option::None });
        num_arms.push(quote! { _ => ::core::option::Option::None });

        string_props.push(quote! {
            &#name::#ident #params => {
                match prop {
                    #(#string_arms),*
                }
            }
        });
        bool_props.push(quote! {
            &#name::#ident #params => {
                match prop {
                    #(#bool_arms),*
                }
            }
        });
        num_props.push(quote! {
            &#name::#ident #params => {
                match prop {
                    #(#num_arms),*
                }
            }
        });
    }

    string_props.push(quote! { _ => ::core::option::Option::None });

    bool_props.push(quote! { _ => ::core::option::Option::None });

    num_props.push(quote! { _ => ::core::option::Option::None });

    Ok(quote! {
        impl #impl_generics #strum_module_path::EnumProperty for #name #ty_generics #where_clause {
            fn get_str(&self, prop: &str) -> ::core::option::Option<&'static str> {
                match self {
                    #(#string_props),*
                }
            }
            fn get_bool(&self, prop: &str) -> ::core::option::Option<bool> {
                match self {
                    #(#bool_props),*
                }
            }
            fn get_int(&self, prop: &str) -> ::core::option::Option<usize> {
                match self {
                    #(#num_props),*
                }
            }
        }
    })
}
