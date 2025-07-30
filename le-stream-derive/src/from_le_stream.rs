use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn from_le_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let body = impl_body(input.data);
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

fn impl_body(data: Data) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut constructor_fields = TokenStream::new();

    match data {
        Data::Struct(structure) => match structure.fields {
            Fields::Named(fields) => {
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

                tokens
                    .extend(quote! { ::core::option::Option::Some(Self { #constructor_fields }) });
                tokens
            }
            Fields::Unit => {
                quote! { ::core::option::Option::Some( Self {} ) }
            }
            Fields::Unnamed(fields) => {
                for field in fields.unnamed {
                    let item_type = field.ty;

                    constructor_fields.extend(quote! {
                        <#item_type as ::le_stream::FromLeStream>::from_le_stream(&mut bytes)?,
                    });
                }

                tokens.extend(quote! { ::core::option::Option::Some(Self(#constructor_fields)) });
                tokens
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
