use extend::Extend;
use iterator_tokens::IteratorTokens;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Ident, Index};

use crate::{add_trait_bounds, get_repr_type};

mod extend;
mod iterator_tokens;

pub fn to_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let repr_type = get_repr_type(&input);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics, &parse_quote!(::le_stream::ToLeStream));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let ((iterator_statement, iterator_type), tail) = impl_body(input.data, &name, repr_type);
    let expanded = quote! {
        impl #impl_generics ::le_stream::ToLeStream for #name #ty_generics #where_clause {
            type Iter = #iterator_type;

            fn to_le_stream(self) -> <Self as ::le_stream::ToLeStream>::Iter {
                #iterator_statement
            }
        }

        #tail
    };

    proc_macro::TokenStream::from(expanded)
}

fn impl_body(
    data: Data,
    name: &Ident,
    repr_type: Option<Ident>,
) -> ((TokenStream, TokenStream), TokenStream) {
    match data {
        Data::Struct(structure) => (
            iterator_for_fields(&structure.fields, Some(&format_ident!("self"))).into(),
            TokenStream::new(),
        ),
        Data::Enum(enumeration) => {
            let repr_type = repr_type.expect("`#[repr(T)]` is required");
            let mut iterator_enum_variants = TokenStream::new();
            let iterator_enum_name = format_ident!("__{name}ToLeStreamIterator");
            let mut iterator_statement_match_arms = TokenStream::new();
            let mut iterator_enum_iterator_match_arms = TokenStream::new();

            for variant in enumeration.variants {
                let ident = variant.ident;
                let iterator = iterator_for_fields(&variant.fields, None);
                let parameters = parameters_for_fields(&variant.fields);
                let (statement, typ) = iterator.into();
                iterator_enum_variants.extend(quote! {
                    #ident(#typ),
                });
                iterator_statement_match_arms.extend(quote! {
                    Self::#ident #parameters => #iterator_enum_name::#ident(#statement),
                });
                iterator_enum_iterator_match_arms.extend(quote! {
                    #iterator_enum_name::#ident(iterator) => iterator.next(),
                });
            }

            let iterator_statement = quote! {
                #[allow(unsafe_code)]
                // SAFETY: This call is safe, because the macro guarantees that the enum is repr(T).
                let discriminant = unsafe { *::core::ptr::from_ref(self).cast::<#repr_type>() } ;
                let discriminant_iterator = <#repr_type as ::le_stream::ToLeStream>::to_le_stream(discriminant);
                let variant_iterator = match self {
                    #iterator_statement_match_arms
                };
                discriminant_iterator.chain(variant_iterator)
            };
            let iterator_type = quote! {
                ::core::iter::Chain<
                    <#repr_type as ::le_stream::ToLeStream>::Iter,
                    #iterator_enum_name
                >
            };

            let iterator_enum = quote! {
                enum #iterator_enum_name {
                    #iterator_enum_variants
                }
            };

            let iterator_enum_iterator_impl = quote! {
                impl ::core::iter::Iterator for #iterator_enum_name {
                    type Item = u8;

                    fn next(&mut self) -> Option<Self::Item> {
                        match self {
                            #iterator_enum_iterator_match_arms
                        }
                    }
                }
            };

            (
                (iterator_statement, iterator_type),
                quote! {
                    #iterator_enum
                    #iterator_enum_iterator_impl
                },
            )
        }
        Data::Union(_) => unimplemented!(),
    }
}

fn iterator_for_fields(fields: &Fields, prefix: Option<&Ident>) -> IteratorTokens {
    let mut iterator = Option::<IteratorTokens>::None;

    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let ident = field.ident.as_ref().expect("Expected named field!");

                if let Some(prefix) = &prefix {
                    iterator.extend(quote! { #prefix.#ident }, &field.ty);
                } else {
                    iterator.extend(ident, &field.ty);
                }
            }
        }
        Fields::Unit => return IteratorTokens::default(),
        Fields::Unnamed(fields) => {
            for (index, field) in fields.unnamed.iter().enumerate() {
                if let Some(prefix) = prefix {
                    iterator.extend(quote! { #prefix.#index }, &field.ty);
                } else {
                    iterator.extend(Index::from(index), &field.ty);
                }
            }
        }
    }

    iterator.unwrap_or_default()
}

fn parameters_for_fields(fields: &Fields) -> TokenStream {
    let mut idents = TokenStream::new();

    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let ident = field.ident.as_ref().expect("Expected named field!");
                idents.extend(quote! { #ident, });
            }

            quote! { { #idents } }
        }
        Fields::Unit => TokenStream::new(),
        Fields::Unnamed(fields) => {
            for (index, _) in fields.unnamed.iter().enumerate() {
                let ident = format_ident!("param{index}");
                idents.extend(quote! { #ident, });
            }

            quote! { ( #idents ) }
        }
    }
}
