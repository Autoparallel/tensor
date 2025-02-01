use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, Ident};

#[proc_macro_derive(MultilinearMap)]
pub fn multilinear_map_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Type?
    let const_generics: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Const(const_param) => Some(&const_param.ident),
            _ => None,
        })
        .collect();

    // use proc_macro2::{Ident, Span};
    //
    // Span::call_site()
    //
    // pub fn multilinear_map(
    //     &self,
    //     v_0: V<M, F>,
    //     v_1: V<N, F>,
    //     v_2: V<P, F>,
    // )
    let input_params = const_generics.iter().enumerate().map(|(i, ident)| {
        // Is this identifier actually scoped correctly to the function? Or, by using
        // ::new raw, have we just made a global or something? span => scope?
        //
        // let param_name = Ident::new(&format!("v_{}", i), ident.span());
        let param_name = Ident::new(&format!("v_{}", i), Span::call_site());
        quote! { #param_name: Vector<#ident, F> }
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

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    let n: usize = input.to_string().parse().expect("Expected a usize");

    // Generate const generic parameters (N0, N1, ..., N{n-1})
    let const_params = (0..n).map(|i| {
        let ident = Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
        quote! { const #ident: usize, }
    });

    // Collect const_params into a Vec to reuse it
    let const_params_vec: Vec<_> = const_params.collect();

    // Generate const generic parameters (N0, N1, ..., N{n-1})
    let constants = (0..n).map(|i| {
        let ident = Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
        quote! { #ident, }
    });

    // Collect const_params into a Vec to reuse it
    let constants_vec: Vec<_> = constants.collect();

    // Generate the coefficients type (Vector<N0, Vector<N1, ..., Vector<N{n-1},
    // F>>)
    let coefficients_type = (0..n).rev().fold(quote! { F }, |acc, i| {
        let ident = Ident::new(&format!("N{}", i), proc_macro2::Span::call_site());
        quote! { Vector<#ident, #acc> }
    });

    // Add multilinear_map implementation
    let input_params = (0..n).map(|i| {
        let param_name = Ident::new(&format!("v_{}", i), Span::call_site());
        let dim_name = Ident::new(&format!("N{}", i), Span::call_site());
        // Build the nested type for each parameter
        let param_type = (i + 1..n).rev().fold(quote! { F }, |acc, j| {
            let next_dim = Ident::new(&format!("N{}", j), Span::call_site());
            quote! { Vector<#next_dim, #acc> }
        });
        quote! { #param_name: &Vector<#dim_name, #param_type> }
    });

    let loop_indices: Vec<_> = (0..n)
        .map(|i| Ident::new(&format!("i_{}", i), Span::call_site()))
        .collect();

    // Build nested scalar products
    let mut inner_computation = quote! { self.coefficients };
    for (i, _index) in loop_indices.iter().enumerate() {
        let v_name = Ident::new(&format!("v_{}", i), Span::call_site());
        inner_computation = quote! {
            #inner_computation.scalar_product(#v_name)
        };
    }

    // Generate the struct definition and implementations
    let expanded = quote! {
        pub struct Tensor<#(#const_params_vec)* F>
        where
            F: ScalarProduct + Copy + Default,
            F::Inner: Add<Output = F::Inner> + Default + Copy,
        {
            pub coefficients: #coefficients_type,
        }

        impl<#(#const_params_vec)* F> Default for Tensor<#(#constants_vec)* F>
        where
            F: ScalarProduct + Copy + Default,
            F::Inner: Add<Output = F::Inner> + Default + Copy,
        {
            fn default() -> Self {
                Self {
                    coefficients: <#coefficients_type>::default(),
                }
            }
        }

        impl<#(#const_params_vec)* F> core::fmt::Debug for Tensor<#(#constants_vec)* F>
        where
            F: ScalarProduct + Copy + Default + Debug,
            F::Inner: Add<Output = F::Inner> + Default + Copy,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("Tensor")
                    .field("coefficients", &self.coefficients)
                    .finish()
            }
        }

        impl<#(#const_params_vec)* F> core::ops::Add for Tensor<#(#constants_vec)* F>
        where
            F: ScalarProduct + Default + Copy + Add<Output = F>,
            F::Inner: Add<Output = F::Inner> + Default + Copy,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self {
                    coefficients: self.coefficients + other.coefficients,
                }
            }
        }

        impl<#(#const_params_vec)* F> core::ops::Mul<F> for Tensor<#(#constants_vec)* F>
        where
            F: ScalarProduct + core::ops::Mul<Output = F> + Copy + Default + AddAssign,
            F::Inner: Add<Output = F::Inner> + Default + Copy,
        {
            type Output = Self;

            fn mul(self, scalar: F) -> Self::Output {
                Self {
                    coefficients:  self.coefficients * scalar,
                }
            }
        }

        // impl<#(#const_params_vec)* F> Tensor<#(#constants_vec)* F>
        // where
        //     F: ScalarProduct + Default + Copy + AddAssign + Mul<F, Output = F> + Add<Output = F>,
        //     F::Inner: Add<Output = F::Inner> + Default + Copy,
        // {
        //     pub fn multilinear_map(&self, #(#input_params),*) -> F {
        //         #inner_computation
        //     }
        // }
    };

    expanded.into()
}
