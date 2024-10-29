use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, ExprLit, Lit, Meta, Path};

#[proc_macro_derive(ConvertRepr)]
pub fn derive_enum_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let repr_type = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("repr"))
        .and_then(|attr| {
            if let Meta::List(list) = &attr.meta {
                list.parse_args::<Path>().ok()
            } else {
                None
            }
        })
        .expect("ConvertRepr requires #[repr(..)] attribute");

    let data = if let syn::Data::Enum(data) = &input.data {
        data
    } else {
        panic!("ConvertRepr can only be derived for enums");
    };

    let mut variants = Vec::new();
    for variant in &data.variants {
        let variant_ident = &variant.ident;
        if let Some((
            _,
            Expr::Lit(ExprLit {
                lit: Lit::Int(discriminant),
                ..
            }),
        )) = &variant.discriminant
        {
            variants.push((variant_ident, discriminant));
        } else {
            panic!("ConvertRepr requires explicit discriminant value for all variants");
        }
    }

    let from_arms = variants.iter().map(|(variant_ident, discriminant)| {
        quote! {
            #enum_name::#variant_ident => #discriminant
        }
    });

    let try_from_arms = variants.iter().map(|(variant_ident, discriminant)| {
        quote! {
            #discriminant => Ok(#enum_name::#variant_ident)
        }
    });

    let expanded = quote! {
        impl std::convert::From<#enum_name> for #repr_type {
            #[inline]
            fn from(value: #enum_name) -> Self {
                match value {
                    #(#from_arms,)*
                }
            }
        }

        impl std::convert::TryFrom<#repr_type> for #enum_name {
            type Error = #repr_type;

            #[inline]
            fn try_from(value: #repr_type) -> Result<Self, Self::Error> {
                match value {
                    #(#try_from_arms,)*
                    value => Err(value)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
