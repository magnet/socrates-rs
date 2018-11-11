// WIP experimenting

#![feature(extern_crate_item_prelude)]
#![feature(custom_attribute)]
// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::DeriveInput;

use proc_macro::TokenStream;

use socrates::component::*;

#[proc_macro_derive(Component, attributes(provide))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut references = Vec::new();
    if let syn::Data::Struct(ref struct_def) = input.data {
        for f in struct_def.fields.iter() {
            if let syn::Type::Path(ref path) = &f.ty {
                let p = &path.path;
                println!("contents {:?}", f.ident);

                for seg in p.segments.iter() {
                    println!("contents {:?}", seg.ident.to_string());
                    if let syn::PathArguments::AngleBracketed(ref type_params) = seg.arguments {
                        for arg in type_params.args.iter() {
                            if let syn::GenericArgument::Type(tpe) = arg {
                                if let syn::Type::TraitObject(trait_obj) = tpe {
                                    for bound in trait_obj.bounds.iter() {
                                        if let syn::TypeParamBound::Trait(ref trt) = bound {
                                            for trt_seg in trt.path.segments.iter() {
                                                println!(
                                                    "contents {:?}",
                                                    trt_seg.ident.to_string()
                                                );
                                                references.push(Reference {
                                                    name: trt_seg.ident.to_string(),
                                                    cardinality: Cardinality::Mandatory,
                                                    policy: Policy::Static,
                                                    policy_option: PolicyOption::Greedy,
                                                });
                                            }
                                        }
                                    }
                                } else if let syn::Type::Path(ref p) = tpe {
                                    for trt_seg in p.path.segments.iter() {
                                        println!("contents {:?}", trt_seg.ident.to_string());
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
    for attr in attrs.iter() {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::List(l) = meta {
                if l.ident.to_string() == "provide" {
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
        }
    }

    let struct_name = &input.ident;
    let struct_name_as_string = struct_name.to_string();

    // Used for debug!
    let component_def = ComponentDefinition {
        name: struct_name.to_string(),
        provides: provides.clone(),
        references: references.clone(),
    };
    println!("{:?}", component_def);

    let mut quoted_provides = Vec::new();
    for prov in provided.iter() {
        let prov_name = prov.to_string();
        quoted_provides.push(quote! {
            socrates::component::definition::Provide {
                name: #prov_name.to_string(),
            }
        });
    }

    let mut quoted_references = Vec::new();
    for rfe in references.iter() {
        let rfe_name = rfe.name.to_string();
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
        )).ok();
        quoted_references.push(quote! {
            socrates::component::definition::Reference {
                name: #rfe_name.to_string(),
                cardinality: #card,
                policy: #pol,
                policy_option: #pol_opt
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

    let expanded = quote! {
        #service_trait

        impl Component for #struct_name {
            fn get_definition() -> socrates::component::ComponentDefinition {
                socrates::component::ComponentDefinition {
                    name: #struct_name_as_string.to_string(),
                    provides: vec![ #(#quoted_provides),*],
                    references: vec![#(#quoted_references),*],
                }
            }

            fn instantiate() -> #struct_name {
                unimplemented!();
            }
        }
    };

    let r: TokenStream = expanded.into();
    println!("{}", r.to_string());
    r
}
