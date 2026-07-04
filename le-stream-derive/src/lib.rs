//! Derive macros for `le-stream`.
//!
//! The macros implement `le_stream::FromLeStream` and
//! `le_stream::ToLeStream` for structs and explicitly represented enums.
//!
//! Struct fields are encoded in declaration order. Enum variants require
//! `#[repr(T)]` and an explicit discriminant on every variant; the discriminant
//! is encoded as `T`, followed by the byte representation of any associated
//! fields.

use from_le_stream::from_le_stream;
use proc_macro2::Ident;
use syn::{DeriveInput, GenericParam, Generics, TypeParamBound};
use to_le_stream::to_le_stream;

mod from_le_stream;
mod to_le_stream;

/// Derives `le_stream::FromLeStream` for a struct or represented enum.
///
/// For structs, fields are decoded in declaration order. For enums, the input
/// stream starts with the `#[repr(T)]` discriminant. Unknown discriminants return
/// [`None`](Option::None).
#[proc_macro_derive(FromLeStream)]
pub fn derive_from_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_le_stream(input)
}

/// Derives `le_stream::ToLeStream` for a struct or represented enum.
///
/// For enums, the generated implementation writes the variant's declared
/// discriminant using the enum's `#[repr(T)]` type before serializing associated
/// fields.
#[proc_macro_derive(ToLeStream)]
pub fn derive_to_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_le_stream(input)
}

fn add_trait_bounds(mut generics: Generics, trait_name: &TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(trait_name.clone());
        }
    }
    generics
}

fn get_repr_type(input: &DeriveInput) -> Option<Ident> {
    input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .find_map(|attr| attr.parse_args().ok())
}
