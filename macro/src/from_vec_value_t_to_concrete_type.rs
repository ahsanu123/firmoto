use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Ident, parse_macro_input};

pub fn derive(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as DeriveInput);

    // let address = value
    //     .iter()
    //     .find(|pr| pr.name == Some(String::from("address")))
    //     .ok_or(FBRequestParseErr::FieldNotFound)?;
    //
    // let address_val = address
    //     .value
    //     .as_field_u8()
    //     .ok_or(FBRequestParseErr::ValueNotFound)?;
    //
    // request.address = address_val.field;

    let data_struct = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("input for derive from vector T to concrete must be a struct"),
    };

    let fields: Vec<Ident> = data_struct
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();

    let field_vals: Vec<Ident> = data_struct
        .fields
        .iter()
        .map(|field| {
            let field = field.ident.as_ref().unwrap().clone();
            let field_val = format_ident!("{}_val", field.to_string());

            field_val
        })
        .collect();

    let q_field = quote! {
        #(
        // fn some is only for preview, with this we able to do cargo expand
        fn some(){
            let #fields = value
                .iter()
                .find(|pr| pr.name == Some(String::from(#fields)))
                .ok_or(FBRequestParseErr::FieldNotFound)?;

            let #field_vals = #fields
                .value
                .as_field_u8()
                .ok_or(FBRequestParseErr::ValueNotFound)?;
        }
        )*
    };

    q_field.into()
}
