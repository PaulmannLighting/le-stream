//! Derive macros for the `FromLeStream` and `ToLeStream` traits.

use from_le_stream::from_le_stream;
use from_le_stream_tagged::from_le_stream_tagged;
use syn::{GenericParam, Generics, TypeParamBound};
use to_le_stream::to_le_stream;

mod from_le_stream;
mod from_le_stream_tagged;
mod to_le_stream;

/// Derive the `FromLeStream` trait for a struct.
#[proc_macro_derive(FromLeStream)]
pub fn derive_from_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_le_stream(input)
}

/// Derive the `FromLeStreamTagged` trait for a struct.
#[proc_macro_derive(FromLeStreamTagged)]
pub fn derive_from_le_stream_tagged(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_le_stream_tagged(input)
}

/// Derive the `ToLeStream` trait for a struct.
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
