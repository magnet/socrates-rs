#![feature(extern_crate_item_prelude)]
#![feature(custom_attribute)]
extern crate proc_macro;

extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::Item;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());

    let mut parseSvc = false;
    let mut svcs: Vec<String> = Vec::new();
    for item in attr {
        println!("{}", item.to_string());
        match item.to_string().as_ref() {
            "service" => parseSvc = true,
            "services" => parseSvc = true,
            ":" => (),
            x => {
                svcs.push(x.to_string());
                ()
            }
        }
    }

    // let input = parse_macro_input!(input as DeriveInput);

    let item: Item = syn::parse(input).expect("failed to parse input");

    let svcs = svcs
        .iter()
        .map(|x| syn::parse_str::<syn::Expr>(x).expect("damn"));

    let ident = match item {
        syn::Item::Struct(ref s) => s.ident.clone(),
        _ => panic!("Component macro only works on Structs"),
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
        #item

        impl Service for #ident {}


        #[macro_use]
        extern crate query_interface;

        interfaces!(#ident: #(#svcs),*);

    };

    let r: TokenStream = expanded.into();
    println!("{}", r.to_string());
    r
    // input
}
