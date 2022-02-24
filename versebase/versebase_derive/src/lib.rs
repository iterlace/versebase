extern crate proc_macro;

use proc_macro::TokenStream;
use std::any::{Any, TypeId};
use quote::{quote, format_ident, quote_spanned, IdentFragment, ToTokens};
use quote::__private::ext::RepAsIteratorExt;
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

    // TODO: verify "id" column exists
    let field_name: Vec<syn::Ident> = fields
        .iter()
        .map(|field| (&field.ident).clone().unwrap())
        .collect()
        ;
    // TODO: verify "id" column exists
    // let field_name2 = vec!["versebase::datatypes::DType::Int".to_token_stream()];
    // println!("{:?}", field_name2);

    let field_datatype: Vec<syn::Type> = fields
        .iter()
        .map(|field|  (&field.ty).clone())
        .collect()
        ;

    let gen = quote! {

        impl #name {
            fn new(#( #field_name: #field_datatype ),*) -> Self {
                Self { #( #field_name ),* }
            }
        }

        impl TableSchema for #name {

            fn fields() -> std::vec::Vec<String> {
                [ #( std::stringify!(#field_name).to_string() ),*].to_vec()
            }

            fn print_info() {
                #(
                    println!(
                        "field {:?} of type {:?}",
                        std::stringify!(#field_name),
                        std::stringify!(#field_datatype)
                    )
                );*
            }

            fn from_(raw: std::vec::Vec<(String, Box<[u8]>)>) -> Self {
                let map: std::collections::HashMap<String, Box<[u8]>> = raw.into_iter().collect();
                Self {
                    #(
                        #field_name: #field_datatype::from_(map[std::stringify!(#field_name)].deref())
                    ),*
                }
            }

            fn get(&self, field: String) -> Option<versebase::datatypes::DType> {
                match field.as_str() {
                    #(
                        std::stringify!(#field_name) => Some(
                            versebase::datatypes::DType::#field_datatype(self.#field_name.clone())
                        )
                    ),*,
                    _ => None,
                }
            }

            fn get_id(&self) -> i32 {
                self.id.get()
            }

            fn to_map(&self) -> std::collections::HashMap<String, versebase::datatypes::DType> {
                std::collections::HashMap::from([
                    #(
                        (
                            String::from(std::stringify!(#field_name)),
                            versebase::datatypes::DType::#field_datatype(self.#field_name.clone())
                        )
                    ),*
                ])
            }

            fn serialize_to_vec(&self) -> std::vec::Vec<(String, Box<[u8]>)> {
                let mut serialized = std::vec::Vec::<(String, Box<[u8]>)>::new();
                #(
                    serialized.push((
                        String::from(std::stringify!(#field_name)),
                        self.#field_name.serialize()
                    ));
                );*

                return serialized;
            }

            fn serialize_to_map(&self) -> std::collections::HashMap<String, Box<[u8]>> {
                let mut v = self.serialize_to_vec();
                let map: std::collections::HashMap<String, Box<[u8]>> = v.into_iter().collect();

                map
            }
        }

        impl std::fmt::Display for #name {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let parts: std::vec::Vec<String> = vec![
                    #(
                        format!(
                            "{}: {:?}",
                            std::stringify!(#field_name),
                            self.#field_name.get()
                        )
                    ),*
                ];

                write!(f, "{}{{ {} }}", stringify!(#name), parts.join(", "))
            }

        }

    };
    eprintln!("{}", gen.to_string());
    gen.into()
}