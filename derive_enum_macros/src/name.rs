// signed-vimrc
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
use syn::Data;

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
            quote! { #enum_name::#ident => #variant_str }
        });

    quote! {
        impl #enum_name {
            fn name(&self) -> &'static str {
                match self {
                    #(#cases),*
                }
            }
        }
    }
}
