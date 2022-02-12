extern crate proc_macro;

use proc_macro::TokenStream;
use std::any::{Any, TypeId};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, LitStr, Token, Ident};


#[proc_macro_derive(TableSchema)]
pub fn table_schema_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_table_schema(&ast)
}


fn impl_table_schema(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    // TODO: ensure only Int, Str, DateTime passed
    for field in fields {
        match &field.ty {
            syn::Type::Verbatim(_) => (),
            syn::Type::Path(_) => (),
            e => panic!("expected raw type, got {}", quote!(#e).to_string())
        };
    }

    let field_name: Vec<syn::Ident> = fields
        .iter()
        .map(|field| (&field.ident).clone().unwrap())
        .collect()
    ;
    let field_type: Vec<syn::Type> = fields
        .iter()
        .map(|field|  (&field.ty).clone())
        .collect()
        ;

    let gen = quote! {

        impl TableSchema for #name {
            fn info() {
                #(
                    println!(
                        "field {:?} of type {:?}",
                        std::stringify!(#field_name),
                        std::stringify!(#field_type)
                    )
                );*
            }

            fn serialize(&self) -> std::vec::Vec<(String, Box<[u8]>)> {
                let mut serialized = std::vec::Vec::<(String, Box<[u8]>)>::new();
                #(
                    serialized.push((
                        String::from(std::stringify!(#field_name)),
                        self.#field_name.serialize()
                    ));
                );*

                return serialized;
            }
        }
    };
    eprintln!("{}", gen.to_string());
    gen.into()
}