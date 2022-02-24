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
        syn::Data::Struct(ref struct_data) => proto_meta_derive_message(&ast, struct_data),
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

enum FieldType {
    Optional,
    BoxedOptional,
    Repeated,
    Primitive,
}

fn is_repeated(typ: &syn::Type) -> FieldType {
    if let syn::Type::Path(path) = typ {
        if let Some(last) = path.path.segments.last() {
            if last.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(ref args) = last.arguments {
                    if let syn::GenericArgument::Type(syn::Type::Path(path2)) =
                        args.args.first().unwrap()
                    {
                        if path2.path.segments.last().unwrap().ident == "Box" {
                            return FieldType::BoxedOptional;
                        } else {
                            return FieldType::Optional;
                        }
                    }
                }
                panic!("Option without type argument?");
            } else if last.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(ref args) = last.arguments {
                    if let syn::GenericArgument::Type(syn::Type::Path(path2)) =
                        args.args.first().unwrap()
                    {
                        if path2.path.segments.last().unwrap().ident == "u8" {
                            return FieldType::Primitive;
                        } else {
                            return FieldType::Repeated;
                        }
                    }
                }
                panic!("Vec without type argument?");
            }
        }
    }
    FieldType::Primitive
}

fn proto_meta_derive_message(ast: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let parse_unknown_matches: Vec<_> = data
        .fields
        .iter()
        .map(|field| {
            if let Some(ident) = &field.ident {
                let action = match is_repeated(&field.ty) {
                    FieldType::Optional => quote! {
                        crate::tree::push_proto_field(
                            self,
                            y,
                            &self.#ident.as_ref(),
                            stringify!(#ident),
                            true,
                            |_, _| Ok(()),
                            |_, _, _| Ok(()),
                        );
                    },
                    FieldType::BoxedOptional => quote! {
                        crate::tree::push_proto_field(
                            self,
                            y,
                            &self.#ident,
                            stringify!(#ident),
                            true,
                            |_, _| Ok(()),
                            |_, _, _| Ok(()),
                        );
                    },
                    FieldType::Repeated => quote! {
                        crate::tree::push_proto_repeated_field(
                            self,
                            y,
                            &self.#ident.as_ref(),
                            stringify!(#ident),
                            true,
                            |_, _| Ok(()),
                            |_, _, _, _| Ok(()),
                        );
                    },
                    FieldType::Primitive => quote! {
                        use crate::proto::meta::ProtoPrimitive;
                        if !y.config.ignore_unknown_fields_set_to_default || !self.#ident.proto_primitive_is_default() {
                            crate::tree::push_proto_field(
                                self,
                                y,
                                &Some(&self.#ident),
                                stringify!(#ident),
                                true,
                                |_, _| Ok(()),
                                |_, _, _| Ok(()),
                            );
                        }
                    },
                };
                quote! {
                    if !y.breadcrumb.fields_parsed.contains(stringify!(#ident)) {
                        unknowns = true;
                        #action
                    }
                }
            } else {
                panic!("protobuf message fields must have names");
            }
        })
        .collect();

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
            fn proto_type_to_node() -> crate::tree::Node {
                use crate::proto::meta::ProtoMessage;
                crate::tree::NodeType::ProtoMessage(Self::proto_message_type()).into()
            }

            fn proto_data_to_node(&self) -> crate::tree::Node {
                use crate::proto::meta::ProtoMessage;
                crate::tree::NodeType::ProtoMessage(Self::proto_message_type()).into()
            }

            fn proto_data_variant(&self) -> Option<&'static str> {
                None
            }

            fn proto_parse_unknown(
                &self,
                y: &mut crate::context::Context<'_>,
            ) -> bool {
                let mut unknowns = false;
                #(#parse_unknown_matches)*
                unknowns
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
            quote! { #name::#ident (_) => #proto_name }
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

    let parse_unknown_matches: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            quote! { #name::#ident (x) => x.proto_parse_unknown(y) }
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
            fn proto_type_to_node() -> crate::tree::Node {
                crate::tree::NodeType::ProtoMissingOneOf.into()
            }

            fn proto_data_to_node(&self) -> crate::tree::Node {
                match self {
                    #(#node_matches),*
                }
            }

            fn proto_data_variant(&self) -> Option<&'static str> {
                use crate::proto::meta::ProtoOneOf;
                Some(self.proto_one_of_variant())
            }

            fn proto_parse_unknown(
                &self,
                y: &mut crate::context::Context<'_>,
            ) -> bool {
                match self {
                    #(#parse_unknown_matches),*
                }
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
            quote! { #name::#ident => #proto_name }
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
                #first_variant_name
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
