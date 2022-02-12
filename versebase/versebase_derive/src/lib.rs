extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, LitStr, Token};


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
    let field_name = fields.iter().map(|field| &field.ident);
    // let field_type = fields.iter().map(|field| &field.ty);

    let gen = quote! {
        impl TableSchema for #name {
            fn info() {
                #(
                    println!("field: {:?}", std::stringify!(#field_name));
                );*
            }
        }
    };
    gen.into()
}