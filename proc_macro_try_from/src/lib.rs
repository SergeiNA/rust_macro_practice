extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(FromPrimitive)]
pub fn derive(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);
    let name = &macro_input.ident;
    let variants = match macro_input.data {
        Data::Enum(ref data_enum) => &data_enum.variants,
        _ => panic!("#[derive(FromPrimitive)] only works on enums"),
    };

    let match_arms = variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        quote! {
            #i => Ok(#name::#ident),
        }
    });

    quote!(
        impl std::convert::TryFrom<usize> for #name {
            type Error = ();
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                match value {
                    #(#match_arms)*
                    _ => Err(()),
                }
            }
        }
    )
    .into()
}
