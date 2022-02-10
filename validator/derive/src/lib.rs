extern crate proc_macro;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro::TokenStream;
use quote::quote;

#[doc(hidden)]
#[proc_macro_derive(ProtoMeta, attributes(proto_meta))]
pub fn proto_meta(input: TokenStream) -> TokenStream {
    proto_meta_derive(syn::parse_macro_input!(input))
}

fn proto_meta_derive(ast: syn::DeriveInput) -> TokenStream {
    match ast.data {
        syn::Data::Struct(_) => proto_meta_derive_message(&ast),
        syn::Data::Enum(ref enum_data) => match enum_data.variants.iter().next().unwrap().fields {
            syn::Fields::Unit => {
                for variant in enum_data.variants.iter() {
                    if !matches!(variant.fields, syn::Fields::Unit) {
                        panic!("all variants of a protobuf oneof enum must have a single, unnamed field");
                    }
                }

                proto_meta_derive_enum(&ast, enum_data)
            }
            syn::Fields::Unnamed(..) => {
                for variant in enum_data.variants.iter() {
                    if let syn::Fields::Unnamed(fields) = &variant.fields {
                        if fields.unnamed.len() != 1 {
                            panic!("all variants of a protobuf oneof enum must have a single, unnamed field");
                        }
                    } else {
                        panic!("all variants of a protobuf oneof enum must have a single, unnamed field");
                    }
                }

                proto_meta_derive_oneof(&ast, enum_data)
            }
            _ => panic!("enum with named elements don't map to protobuf constructs"),
        },
        syn::Data::Union(_) => panic!("unions don't map to protobuf constructs"),
    }
}

fn proto_meta_derive_message(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote!(
        impl #impl_generics crate::proto::meta::ProtoMessage for #name #ty_generics #where_clause {
            fn proto_message_type() -> &'static str {
                use ::once_cell::sync::Lazy;
                static TYPE_NAME: Lazy<::std::string::String> = Lazy::new(|| {
                    let iter = module_path!()
                        .split("::")
                        .skip(2)
                        .chain(::std::iter::once(stringify!(#name)));
                    ::itertools::Itertools::intersperse(iter, ".").collect()
                });
                &TYPE_NAME
            }
        }

        impl #impl_generics crate::proto::meta::ProtoDatum for #name #ty_generics #where_clause {
            fn proto_type_to_node() -> crate::doc_tree::Node {
                use crate::proto::meta::ProtoMessage;
                crate::doc_tree::NodeType::ProtoMessage(Self::proto_message_type()).into()
            }

            fn proto_data_to_node(&self) -> crate::doc_tree::Node {
                use crate::proto::meta::ProtoMessage;
                crate::doc_tree::NodeType::ProtoMessage(Self::proto_message_type()).into()
            }

            fn proto_data_variant(&self) -> Option<&'static str> {
                None
            }
        }
    )
    .into()
}

fn proto_meta_derive_oneof(ast: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let variant_matches: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let proto_name = ident.to_string().to_snake_case();
            quote! { #name::#ident (_) => stringify!(#proto_name) }
        })
        .collect();

    let node_matches: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            quote! { #name::#ident (x) => x.proto_data_to_node() }
        })
        .collect();

    quote!(
        impl #impl_generics crate::proto::meta::ProtoOneOf for #name #ty_generics #where_clause {
            fn proto_one_of_variant(&self) -> &'static str {
                match self {
                    #(#variant_matches),*
                }
            }
        }

        impl #impl_generics crate::proto::meta::ProtoDatum for #name #ty_generics #where_clause {
            fn proto_type_to_node() -> crate::doc_tree::Node {
                crate::doc_tree::NodeType::ProtoMissingOneOf.into()
            }

            fn proto_data_to_node(&self) -> crate::doc_tree::Node {
                match self {
                    #(#node_matches),*
                }
            }

            fn proto_data_variant(&self) -> Option<&'static str> {
                use crate::proto::meta::ProtoOneOf;
                Some(self.proto_one_of_variant())
            }
        }
    )
    .into()
}

fn proto_meta_derive_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let upper_name = name.to_string().to_shouty_snake_case();

    let variant_names: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let proto_name = format!(
                "{}_{}",
                upper_name,
                ident.to_string().to_shouty_snake_case()
            );
            (ident, proto_name)
        })
        .collect();

    let variant_matches: Vec<_> = variant_names
        .iter()
        .map(|(ident, proto_name)| {
            quote! { #name::#ident => stringify!(#proto_name) }
        })
        .collect();

    let (_, first_variant_name) = &variant_names[0];

    quote!(
        impl #impl_generics crate::proto::meta::ProtoEnum for #name #ty_generics #where_clause {
            fn proto_enum_type() -> &'static str {
                use ::once_cell::sync::Lazy;
                static TYPE_NAME: Lazy<::std::string::String> = Lazy::new(|| {
                    let iter = module_path!()
                        .split("::")
                        .skip(2)
                        .chain(::std::iter::once(stringify!(#name)));
                    ::itertools::Itertools::intersperse(iter, ".").collect()
                });
                &TYPE_NAME
            }

            fn proto_enum_default_variant() -> &'static str {
                stringify!(#first_variant_name)
            }

            fn proto_enum_variant(&self) -> &'static str {
                match self {
                    #(#variant_matches),*
                }
            }
        }
    )
    .into()
}

/*let variant_name_impl = if let syn::Data::Enum(ref data_enum) = ast.data {

    let matches: Vec<_> = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let proto_ident = ident.to_string().to_snake_case();

        let params = match variant.fields {
            syn::Fields::Unit => quote!{},
            syn::Fields::Unnamed(..) => quote!{(..)},
            syn::Fields::Named(..) => quote!{{..}}
        };

        quote!{ #name::#ident #params => stringify!(#proto_ident)}

    }).collect();

    quote!{
        match *self {
            #(#matches),*
        }
    }

} else {

    quote!{
        panic!("not an enum!");
    }

};

quote!(
    impl #impl_generics crate::ProtoNames for #name #ty_generics #where_clause {
        fn proto_type_name() -> &'static str {
            use ::once_cell::sync::Lazy;
            static TYPE_NAME: Lazy<::std::string::String> = Lazy::new(|| {
                let iter = module_path!()
                    .split("::")
                    .skip(2)
                    .chain(::std::iter::once(stringify!(#name)));
                ::itertools::Itertools::intersperse(iter, ".").collect()
            });
            &TYPE_NAME
        }

        fn proto_variant_name(&self) -> &'static str {
            #variant_name_impl
        }
    }
)
.into()*/
