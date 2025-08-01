use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident,
};

use crate::add_trait_bounds;

pub fn from_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let repr_type: Option<Ident> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .find_map(|attr| attr.parse_args().ok());
    let name = input.ident;
    let generics = add_trait_bounds(input.generics, &parse_quote!(::le_stream::FromLeStream));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = impl_body(input.data, repr_type);
    let expanded = quote! {
        impl #impl_generics ::le_stream::FromLeStream for #name #ty_generics #where_clause {
            fn from_le_stream<__LeStreamBytesIterator>(mut bytes: __LeStreamBytesIterator) -> ::core::option::Option<Self>
            where
                __LeStreamBytesIterator: ::core::iter::Iterator<Item = u8>
            {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn impl_body(data: Data, repr_type: Option<Ident>) -> TokenStream {
    match data {
        Data::Struct(structure) => impl_fields(structure.fields, &quote! { Self }),
        Data::Enum(enumeration) => {
            let repr_type = repr_type.expect("`#[repr(T)]` is required");
            let mut tokens = quote! {
                let discriminant = <#repr_type as ::le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
            };
            let mut match_arms = TokenStream::new();

            for variant in enumeration.variants {
                let (_, discriminant) = variant
                    .discriminant
                    .expect("enum variant has no discriminant");
                let variant_ident = variant.ident;
                let match_arm_body = impl_fields(variant.fields, &quote! { Self::#variant_ident });
                match_arms.extend(quote! {
                    #discriminant => {
                        #match_arm_body
                    }
                });
            }

            tokens.extend(quote! {
                match discriminant {
                    #match_arms
                    _ => ::core::option::Option::None,
                }
            });

            tokens
        }
        Data::Union(_) => unimplemented!(),
    }
}

fn impl_fields(fields: Fields, constructor: &TokenStream) -> TokenStream {
    match fields {
        Fields::Named(fields) => impl_fields_named(fields, constructor),
        Fields::Unnamed(fields) => impl_fields_unnamed(fields, constructor),
        Fields::Unit => quote! { ::core::option::Option::Some( #constructor ) },
    }
}

fn impl_fields_named(fields: FieldsNamed, constructor: &TokenStream) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut constructor_fields = TokenStream::new();

    for field in fields.named {
        let item_name = field.ident.clone().expect("struct field has no name");
        let item_type = field.ty;

        tokens.extend(quote! {
            let #item_name = <#item_type as ::le_stream::FromLeStream>::from_le_stream(&mut bytes)?;
        });

        constructor_fields.extend(quote! {
            #item_name,
        });
    }

    tokens.extend(quote! { ::core::option::Option::Some( #constructor { #constructor_fields }) });
    tokens
}

fn impl_fields_unnamed(fields: FieldsUnnamed, constructor: &TokenStream) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut attributes = TokenStream::new();

    for field in fields.unnamed {
        let item_type = field.ty;

        attributes.extend(quote! {
            <#item_type as ::le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
        });
    }

    tokens.extend(quote! { ::core::option::Option::Some( #constructor ( #attributes ) ) });
    tokens
}
