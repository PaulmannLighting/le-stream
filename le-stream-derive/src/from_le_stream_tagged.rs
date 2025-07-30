use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn from_le_stream_tagged(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tag_type: Ident = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .find_map(|attr| attr.parse_args().ok())
        .expect("`#[repr(T)]` is required");
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let body = impl_body(input.data);
    let expanded = quote! {
        impl #impl_generics ::le_stream::FromLeStreamTagged for #name #ty_generics #where_clause {
            type Tag = #tag_type;

            fn from_le_stream_tagged<__LeStreamBytesIterator>(tag: <Self as ::le_stream::FromLeStreamTagged>::Tag, mut bytes: __LeStreamBytesIterator) -> ::core::result::Result<::core::option::Option<Self>, <Self as ::le_stream::FromLeStreamTagged>::Tag>
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
    match data {
        Data::Enum(data_enum) => {
            let mut variants = TokenStream::new();

            for variant in data_enum.variants {
                let ident = variant.ident;

                let Some((_, expr)) = variant.discriminant else {
                    panic!("enum variant {ident} has no discriminant");
                };

                match variant.fields {
                    Fields::Unit => variants.extend(quote! {
                        #expr => Ok(Some(Self::#ident)),
                    }),
                    Fields::Unnamed(fields) => {
                        let mut field_values = TokenStream::new();

                        for field in fields.unnamed {
                            let typ = field.ty;
                            field_values.extend(quote! {
                                if let ::core::option::Option::Some(value) = <#typ as ::le_stream::FromLeStream>::from_le_stream(&mut bytes) {
                                    value
                                } else {
                                    return ::core::result::Result::Ok(::core::option::Option::None);
                                }
                            });
                        }

                        variants.extend(quote! {
                            #expr => Ok(Some(Self::#ident(#field_values))),
                        });
                    }
                    Fields::Named(fields) => {
                        let mut field_values = TokenStream::new();

                        for (ident, field) in fields
                            .named
                            .into_iter()
                            .filter_map(|field| field.ident.clone().map(|ident| (ident, field)))
                        {
                            let typ = field.ty;
                            field_values.extend(quote! {
                                #ident: if let ::core::option::Option::Some(value) = <#typ as ::le_stream::FromLeStream>::from_le_stream(&mut bytes) {
                                    value
                                } else {
                                    return ::core::result::Result::Ok(::core::option::Option::None);
                                },
                            });
                        }

                        variants.extend(quote! {
                            #expr => Ok(Some(Self::#ident { #field_values } )),
                        });
                    }
                }
            }

            variants.extend(quote! {
                __invalid_tag => Err(__invalid_tag),
                __invalid_tag => Err(__invalid_tag),
            });

            quote! {
                match tag {
                    #variants
                }
            }
        }
        _ => unimplemented!(),
    }
}
