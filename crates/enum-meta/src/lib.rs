use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, DeriveInput, Expr, ExprLit, Ident, Lit, Meta,
    MetaNameValue, Token,
};

#[proc_macro_derive(EnumMeta, attributes(meta_attrs, meta))]
pub fn derive_enum_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let cattr_names = input
        .attrs
        .iter()
        .find(|cattr| cattr.path().is_ident("meta_attrs"))
        .map(|cattr| {
            if let Meta::List(list) = &cattr.meta {
                list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<_>>()
            } else {
                panic!("meta_attrs must be comma separated list of identifiers");
            }
        })
        .unwrap_or_default();

    let data = if let syn::Data::Enum(data) = &input.data {
        data
    } else {
        panic!("EnumMeta can only be derived for enums");
    };

    let mut cattr_match_arms = vec![Vec::new(); cattr_names.len()];

    for variant in &data.variants {
        let variant_ident = &variant.ident;

        if let Some(attr) = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("meta"))
        {
            if let Meta::List(list) = &attr.meta {
                let nested_metas = list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .unwrap();

                for meta in nested_metas {
                    if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
                        if let Some(cattr_index) =
                            cattr_names.iter().position(|cattr| path.is_ident(cattr))
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            }) = value
                            {
                                cattr_match_arms[cattr_index].push(quote! {
                                    #enum_name::#variant_ident => #s
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let methods = cattr_names
        .iter()
        .zip(cattr_match_arms.iter())
        .map(|(cattr_name, arms)| {
            let doc = format!("Get {} meta for this variant", cattr_name);
            quote! {
                #[doc = #doc]
                pub fn #cattr_name(&self) -> &'static str {
                    match self {
                        #(#arms,)*
                    }
                }
            }
        });

    let expanded = quote! {
        impl #enum_name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}
