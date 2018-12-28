// WIP experimenting

//#![feature(extern_crate_item_prelude)]
//#![feature(custom_attribute)]
// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::DeriveInput;

use proc_macro::TokenStream;

use socrates_core::component::*;

struct ReferenceInfo {
    pub name: String,
    pub unqualified_svc_name: String,
    pub cardinality: Cardinality,
    pub policy: Policy,
    pub policy_option: PolicyOption,
}

#[proc_macro_derive(Component, attributes(provide, custom_lifecycle))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut references = Vec::new();
    if let syn::Data::Struct(ref struct_def) = input.data {
        for f in struct_def.fields.iter() {
            if let syn::Type::Path(ref path) = &f.ty {
                let p = &path.path;
                println!("base path {:?}", f.ident);

                for seg in p.segments.iter() {
                    println!("segment {:?}", seg.ident.to_string());
                    if let syn::PathArguments::AngleBracketed(ref type_params) = seg.arguments {
                        for arg in type_params.args.iter() {
                            if let syn::GenericArgument::Type(tpe) = arg {
                                if let syn::Type::TraitObject(trait_obj) = tpe {
                                    for bound in trait_obj.bounds.iter() {
                                        if let syn::TypeParamBound::Trait(ref trt) = bound {
                                            for trt_seg in trt.path.segments.iter() {
                                                println!(
                                                    "trait {:?}",
                                                    trt_seg.ident.to_string()
                                                );
                                                references.push(ReferenceInfo {
                                                    name: f
                                                        .ident
                                                        .as_ref()
                                                        .map(|x| x.to_string())
                                                        .unwrap_or_else(|| {
                                                            trt_seg.ident.to_string()
                                                        }),
                                                    unqualified_svc_name: trt_seg.ident.to_string(),
                                                    cardinality: Cardinality::Mandatory,
                                                    policy: Policy::Static,
                                                    policy_option: PolicyOption::Greedy,
                                                });
                                            }
                                        }
                                    }
                                } else if let syn::Type::Path(ref p) = tpe {
                                    for trt_seg in p.path.segments.iter() {
                                        println!("path {:?}", trt_seg.ident.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let attrs = input.attrs;

    // Find services defined with #[provide(DynServiceTrait1, DynServiceTrait2)]
    let mut provides = Vec::new(); // for ComponentDefinition
    let mut provided = Vec::new(); // layout for query_interface macro.
    let mut implement_lifecycle = true;
    for attr in attrs.iter() {
        if let Ok(meta) = attr.parse_meta() {
            match meta {
                syn::Meta::List(l) => {
                    let ident = l.ident.to_string();
                    if ident == "provide" {
                        for nested in l.nested.iter() {
                            if let syn::NestedMeta::Meta(syn::Meta::Word(svc_name)) = nested {
                                provides.push(Provide {
                                    name: svc_name.to_string(),
                                });
                                provided.push(svc_name.clone());
                            }
                        }
                    }
                }
                syn::Meta::Word(ident) => {
                    let ident = ident.to_string();
                    if ident == "custom_lifecycle" {
                        implement_lifecycle = false;
                    }
                }
                _ => (),
            }
        }
    }

    let struct_name = &input.ident;
    let struct_name_as_string = struct_name.to_string();

    let mut quoted_provides = Vec::new();
    for prov in provided.iter() {
        let prov_name = syn::parse_str::<syn::Expr>(&prov.to_string()).ok();;
        quoted_provides.push(quote! {
            socrates::component::definition::Provide {
                name: socrates::service::Service::get_name::<#prov_name>().to_string(),
            }
        });
    }

    let mut quoted_references = Vec::new();
    for rfe in references.iter() {
        let rfe_name = rfe.name.to_string();
        let rfe_svc_name = syn::parse_str::<syn::Expr>(&rfe.unqualified_svc_name.to_string()).ok();

        let card = syn::parse_str::<syn::Path>(&format!(
            "socrates::component::definition::Cardinality::{:?}",
            rfe.cardinality
        ))
        .ok();
        let pol = syn::parse_str::<syn::Path>(&format!(
            "socrates::component::definition::Policy::{:?}",
            rfe.policy
        ))
        .ok();
        let pol_opt = syn::parse_str::<syn::Path>(&format!(
            "socrates::component::definition::PolicyOption::{:?}",
            rfe.policy_option
        ))
        .ok();
        quoted_references.push(quote! {
            socrates::component::definition::Reference {
                name: #rfe_name.to_string(),
                svc_name: socrates::service::Service::get_name::<#rfe_svc_name>().into(),
                svc_query: socrates::service::query::ServiceQuery::by_type_id(socrates::service::Service::type_id::<#rfe_svc_name>()),
                options: socrates::component::definition::ReferenceOptions {
                    cardinality: #card,
                    policy: #pol,
                    policy_option: #pol_opt
                }
            }
        });
    }

    let service_trait = if provided.is_empty() {
        None
    } else {
        Some(quote! {
            use socrates::service::Service;
            impl Service for #struct_name {}

            #[macro_use]
            extern crate query_interface;
            interfaces!(#struct_name: #(#provided),*);

        })
    };

    let lifecycle_trait = if implement_lifecycle {
        Some(quote! {
            impl socrates::component::Lifecycle for #struct_name {

            }
        })
    } else {
        None
    };

    let expanded = quote! {
        #service_trait
        impl socrates::component::Component for #struct_name {
            fn get_definition() -> socrates::component::ComponentDefinition {
                socrates::component::ComponentDefinition {
                    name: #struct_name_as_string.to_string(),
                    provides: vec![ #(#quoted_provides),*],
                    references: vec![#(#quoted_references),*],
                }
            }

            fn instantiate(ctx: &socrates::module::Context, references: &socrates::component::ComponentReferences) -> Option<#struct_name> {
                println!("Instanciating me, {}, unimplemented!", #struct_name_as_string);
                unimplemented!();
            }

            fn update(&self, field_id: usize, ctx: &socrates::module::Context,
                    references: &socrates::component::ComponentReferences,
                ) -> Option<()> {
                    println!("I'm updated");
                    Some(())
            }
        }

        #lifecycle_trait
    };

    let r: TokenStream = expanded.into();
    println!("{}", r.to_string());
    r
}

#[proc_macro_attribute]
pub fn service_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: syn::ItemTrait = parse_macro_input!(item);

    let svc_trait_path: syn::Path = syn::parse_str("socrates::service::Service").unwrap();

    let svc_trait_bound = syn::TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: svc_trait_path,
    };

    input
        .supertraits
        .push(syn::TypeParamBound::Trait(svc_trait_bound));

    let trait_name = &input.ident;
    let trait_name_as_string = trait_name.to_string();

    let expanded = quote! {
        #input

        impl socrates::service::Named for #trait_name {
            fn type_name() -> &'static str {
                concat!(module_path!(), "::", #trait_name_as_string)
            }
        }
    };

    let r: TokenStream = expanded.into();
    println!("{}", r.to_string());
    r
}
