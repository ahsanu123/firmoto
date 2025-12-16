use proc_macro::TokenStream;

mod from_vec_value_t_to_concrete_type;

#[proc_macro_derive(FromVecTToConcrete)]
pub fn from_t_to_concrete(input: TokenStream) -> TokenStream {
    from_vec_value_t_to_concrete_type::derive(input)
}
