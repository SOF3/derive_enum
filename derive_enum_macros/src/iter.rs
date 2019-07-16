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

use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{Data, Fields, Ident};

pub fn derive(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let data = match &input.data {
        Data::Enum(e) => e,
        _ => panic!("Cannot derive for non-enum type"),
    };

    let mut i = 0u32;
    let cases = data.variants.iter()
        .map(|variant| {
            let ident = &variant.ident;
            let variant_name = Literal::string(&ident.to_string());
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
            i += 1;
            let ord = Literal::u32_suffixed(i);
            quote! { #ord => Some((#variant_name, &|| #enum_name::#ident #fields)) }
        });

    let iter_name = Ident::new(&format!("{}AllIter", enum_name.to_string()), Span::call_site());

    quote! {
        impl ::derive_enum::Iter for #enum_name {
            fn all() -> Box<dyn Iterator<Item = (&'static str, &'static dyn Fn() -> Self)>> { Box::new(#iter_name(0)) }
        }

        struct #iter_name(u32);

        impl ::std::iter::Iterator for #iter_name {
            type Item = (&'static str, &'static dyn Fn() -> #enum_name);

            fn next(&mut self) -> Option<Self::Item> {
                self.0 += 1;
                match self.0 {
                    #(#cases,)*
                    _ => None,
                }
            }
        }
    }
}
