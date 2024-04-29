use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Ident};

#[proc_macro_derive(MultilinearMap)]
pub fn multilinear_map_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let const_generics: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Const(const_param) => Some(&const_param.ident),
            _ => None,
        })
        .collect();

    let input_params = const_generics.iter().enumerate().map(|(i, ident)| {
        let param_name = Ident::new(&format!("v_{}", i), ident.span());
        quote! { #param_name: V<#ident, F> }
    });

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn multilinear_map(&self, #(#input_params),*) -> F {
                // Your implementation logic goes here
                todo!()
            }
        }
    };

    TokenStream::from(expanded)
}
