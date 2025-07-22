extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);
    let name = &macro_input.ident;
    let variants = match macro_input.data {
        Data::Enum(ref data_enum) => &data_enum.variants,
        _ => panic!("#[derive(EnumInfo)] only works on enums"),
    };
    let total_variants = variants.len();
    quote!(
        impl #name {
            /// Returns the number of variants in the enum.
            pub fn count() -> usize {
                #total_variants
            }
        }
    )
    .into()
}
