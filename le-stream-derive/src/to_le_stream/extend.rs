use quote::ToTokens;
use syn::Type;

/// Trait for extending iterator tokens with fields and types.
pub trait Extend {
    fn extend<T>(&mut self, field: T, typ: &Type)
    where
        T: ToTokens;
}
