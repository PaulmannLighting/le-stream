use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

use super::extend::Extend;

#[derive(Debug)]
pub struct IteratorTokens {
    statement: TokenStream,
    typ: TokenStream,
}

impl Extend for IteratorTokens {
    fn extend<T>(&mut self, field: T, typ: &Type)
    where
        T: ToTokens,
    {
        self.statement
            .extend(quote! { .chain(<#typ as ::le_stream::ToLeStream>::to_le_stream(#field)) });
        let current_type = &self.typ;
        self.typ =
            quote! { ::core::iter::Chain<#current_type, <#typ as ::le_stream::ToLeStream>::Iter> };
    }
}

impl Default for IteratorTokens {
    fn default() -> Self {
        Self {
            statement: quote! { ::core::iter::empty() },
            typ: quote! { ::core::iter::Empty<u8> },
        }
    }
}

impl From<IteratorTokens> for (TokenStream, TokenStream) {
    fn from(iterator: IteratorTokens) -> Self {
        (iterator.statement, iterator.typ)
    }
}

impl Extend for Option<IteratorTokens> {
    fn extend<T>(&mut self, field: T, typ: &Type)
    where
        T: ToTokens,
    {
        if let Some(iterator) = self {
            iterator.extend(field, typ);
        } else {
            self.replace(IteratorTokens {
                statement: quote! { <#typ as ::le_stream::ToLeStream >::to_le_stream(#field) },
                typ: quote! { <#typ as ::le_stream::ToLeStream >::Iter },
            });
        }
    }
}
