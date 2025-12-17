use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Ident, Type, parse_macro_input};

pub fn derive(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as DeriveInput);
    let struct_name = input.ident;

    let data_struct = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("input for derive from vector T to concrete must be a struct"),
    };

    let fields: Vec<Ident> = data_struct
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();

    let fields_str: Vec<String> = data_struct
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
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

    let val_type: Vec<Ident> = data_struct
        .fields
        .iter()
        .filter_map(|field| {
            if let Type::Path(type_path) = &field.ty {
                let ident = type_path.path.get_ident().cloned().unwrap();
                let val_ty = format_ident!("as_field_{}", ident.to_string().to_lowercase());
                Some(val_ty)
            } else {
                None
            }
        })
        .collect();

    let request_list: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .zip(field_vals.iter())
        .zip(val_type.iter())
        .zip(fields_str.iter())
        .map(|(((field, field_val), val_ty), field_str)| {
            if val_ty.to_string().contains("string") {
                quote! {
                    request.#field = #field_val
                        .field
                        .clone()
                        .ok_or(FBRequestParseErr::StringUnParseable(String::from(#field_str)))?;
                }
            } else {
                quote! {
                    request.#field = #field_val.field;
                }
            }
        })
        .collect();

    let q_fields = quote! {
        impl #struct_name {
            pub fn from(vec_value_t: Vec<ValueT>) -> Result<Self, FBRequestParseErr>
            where
                Self: Default,
            {
                let mut request = Self::default();

                #(
                    let #fields = vec_value_t
                        .iter()
                        .find(|pr| pr.name == Some(String::from(#fields_str)))
                        .ok_or(FBRequestParseErr::FieldNotFound(String::from(#fields_str)))?;

                    let #field_vals = #fields
                        .value
                        .#val_type()
                        .ok_or(FBRequestParseErr::ValueNotFound(String::from(#fields_str)))?;

                    #request_list
                )*

                Ok(request)
            }
        }
    };

    q_fields.into()
}
