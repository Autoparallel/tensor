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

    let loop_indices: Vec<_> = (0..const_generics.len())
        .map(|i| Ident::new(&format!("i_{}", i), proc_macro2::Span::call_site()))
        .collect();

    let component_product = loop_indices.iter().zip(0..).map(|(index, i)| {
        let param_name = Ident::new(&format!("v_{}", i), index.span());
        quote! { * #param_name.0[#index] }
    });

    // Add the calculation to the innermost loop
    let coefficient_access =
        loop_indices
            .iter()
            .fold(quote! { self.coefficients }, |acc, index| {
                quote! { #acc.0[#index] }
            });

    let mut loop_nest = quote! {
        sum += #coefficient_access #(#component_product)*;
    };

    for (index, ident) in loop_indices.iter().rev().zip(const_generics.iter().rev()) {
        loop_nest = quote! {
            for #index in 0..#ident {
                #loop_nest
            }
        };
    }

    loop_nest = quote! {
        #loop_nest

    };

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn multilinear_map(&self, #(#input_params),*) -> F {
                let mut sum = F::default();
                #loop_nest
                sum
            }
        }
    };

    TokenStream::from(expanded)
}
