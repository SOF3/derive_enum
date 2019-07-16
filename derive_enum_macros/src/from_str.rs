// derive-enum
//
// Copyright (C) 2019 chankyin
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro2::Literal;
use quote::quote;
use syn::{Data, Fields};

pub fn derive(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let data = match &input.data {
        Data::Enum(e) => e,
        _ => panic!("Cannot derive for non-enum type"),
    };
    let cases = data.variants.iter()
        .map(|variant| {
            let ident = &variant.ident;
            let variant_str = Literal::string(&ident.to_string());
            let fields = match &variant.fields {
                Fields::Named(fields) => {
                    let mapped = fields.named.iter().map(|field| {
                        let name = match &field.ident {
                            Some(name) => name,
                            None => panic!("Named fields should have names"),
                        };
                        let ty = &field.ty;
                        quote!( #name: #ty::default() )
                    });
                    quote!( { #(#mapped),* } )
                },
                Fields::Unnamed(fields) => {
                    let mapped = fields.unnamed.iter().map(|field| &field.ty );
                    quote!{ ( #(#mapped::default()),* ) }
                },
                Fields::Unit => quote!(),
            };
            quote! { #variant_str => Ok(#enum_name::#ident #fields) }
        });

    quote! {
        impl std::str::FromStr for #enum_name {
            type Err = ::derive_enum::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#cases,)*
                    _ => Err(derive_enum::Error::NoSuchEnum),
                }
            }
        }
    }
}
