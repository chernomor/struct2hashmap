// vim: set expandtab ts=4 sw=4:
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// https://docs.rs/syn/
// https://docs.rs/quote/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToHashMap)]
pub fn to_hashmap(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(datastruct) => match datastruct.fields {
            syn::Fields::Named(fields) => {
                fields.named.clone()
            },
            _ => panic!("You can only derive this on structs with named fields!"),
        },
        _ => panic!("You can only derive this on structs with named fields!"),
    };
    let inserts = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let key = ident.to_string();
        quote! {
            hm.insert(#key.to_string(), format!("{}", self.#ident))
        }
    });

    let inserts_prefix = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let key = ident.to_string();
        quote! {
            hm.insert(format!("{}{}", prefix, #key.to_string()), format!("{}", self.#ident))
        }
    });

    let tokens = quote! {

        impl ToHashMap for #struct_name {
            fn to_hashmap(&self) -> HashMap<String, String> {
                let mut hm = HashMap::new();
                #( #inserts; )*
                hm
            }
            fn to_hashmap_with_prefix(&self, prefix: &str) -> HashMap<String, String> {
                let mut hm = HashMap::new();
                #( #inserts_prefix; )*
                hm
            }
        }
    };

    tokens.into()
}

