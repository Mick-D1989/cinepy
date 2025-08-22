use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Path, parse_macro_input};

/// Attribute: #[from_packed(PackedType)]
#[proc_macro_attribute]
pub fn from_packed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let packed_ty = parse_macro_input!(attr as Path);
    let input = parse_macro_input!(item as ItemStruct);
    let target_ident = &input.ident;

    // Generate: field: unsafe { ptr::read_unaligned(addr_of!(packed.field)) }
    let fields = input.fields.iter().map(|f| {
        let name = f.ident.as_ref().expect("named fields only");
        quote! {
            #name: unsafe {
                core::ptr::read_unaligned(core::ptr::addr_of!(packed.#name))
            }
        }
    });

    // We implement From<&PackedType> to avoid moving a huge struct by value.
    let expanded = quote! {
        #input

        impl From<& #packed_ty> for #target_ident {
            #[inline(always)]
            fn from(packed: & #packed_ty) -> Self {
                Self { #(#fields),* }
            }
        }

        // Optional convenience impl; calls the &T version to avoid code dup.
        impl From<#packed_ty> for #target_ident
        where
            #packed_ty: Copy,
        {
            #[inline(always)]
            fn from(packed: #packed_ty) -> Self {
                (&packed).into()
            }
        }
    };

    TokenStream::from(expanded)
}
