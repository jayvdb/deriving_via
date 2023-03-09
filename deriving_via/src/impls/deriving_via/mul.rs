use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let generics = {
        let lt = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt = &input.generics.gt_token;

        quote! { #lt #params #gt }
    };
    let generic_params = {
        let lt = &input.generics.lt_token;
        let params = input.generics.params.iter().filter_map(|p| match p {
            GenericParam::Type(ty) => Some(&ty.ident),
            _ => None,
        });
        let gt = &input.generics.gt_token;

        quote! { #lt #(#params),* #gt }
    };
    let where_clause = &input.generics.where_clause;
    let (accessor, _, constructor) = extract_fields(input);

    via.map_or_else(
        || {
            quote! {
                impl #generics std::ops::Mul for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn mul(self, other: Self) -> Self {
                        #constructor((self.#accessor * other.#accessor).into())
                    }
                }
                impl #generics std::ops::Div for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn div(self, other: Self) -> Self {
                        #constructor((self.#accessor / other.#accessor).into())
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #generics std::ops::Mul for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn mul(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() * rhs.to_owned()).into()
                    }
                }
                impl #generics std::ops::Div for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn div(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() / rhs.to_owned()).into()
                    }
                }
            }
        },
    )
}