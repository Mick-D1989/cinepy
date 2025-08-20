use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Path, parse_macro_input};

/// Attribute: #[from_packed(PackedType)]
#[proc_macro_attribute]
pub fn from_packed(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute as a type path
    let packed_ty = parse_macro_input!(attr as Path);

    // Parse the struct definition
    let input = parse_macro_input!(item as ItemStruct);
    let target_ident = &input.ident;

    // Generate field-by-field assignments
    let fields: Vec<_> = input
        .fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            quote! { #name: packed.#name }
        })
        .collect();

    let expanded = quote! {
        #input

        impl From<#packed_ty> for #target_ident {
            fn from(packed: #packed_ty) -> Self {
                Self { #(#fields),* }
            }
        }
    };

    TokenStream::from(expanded)
}
