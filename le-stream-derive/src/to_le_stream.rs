use extend::Extend;
use iterator_tokens::IteratorTokens;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Index};

use crate::add_trait_bounds;

mod extend;
mod iterator_tokens;

pub fn to_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics, &parse_quote!(::le_stream::ToLeStream));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (iterator_statement, iterator_type) = impl_body(input.data).into();
    let expanded = quote! {
        impl #impl_generics ::le_stream::ToLeStream for #name #ty_generics #where_clause {
            type Iter = #iterator_type;

            fn to_le_stream(self) -> <Self as ::le_stream::ToLeStream>::Iter {
                #iterator_statement
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn impl_body(data: Data) -> IteratorTokens {
    let mut iterator = Option::<IteratorTokens>::None;

    match data {
        Data::Struct(structure) => match structure.fields {
            Fields::Named(fields) => {
                for (index, field) in fields.named.iter().enumerate() {
                    if let Some(ident) = &field.ident {
                        iterator.extend(ident, &field.ty);
                    } else {
                        iterator.extend(Index::from(index), &field.ty);
                    }
                }
            }
            Fields::Unit => return IteratorTokens::default(),
            Fields::Unnamed(fields) => {
                for (index, field) in fields.unnamed.into_iter().enumerate() {
                    iterator.extend(Index::from(index), &field.ty);
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }

    iterator.unwrap_or_default()
}
