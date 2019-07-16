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

extern crate proc_macro;

macro_rules! derive {
    ($feature: literal: $name: ident, $fn: ident -> $mod: ident) => {
        #[cfg(feature = $feature)]
        mod $mod;
        
        #[cfg(feature = $feature)]
        #[proc_macro_derive($name)]
        pub fn $fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            From::from($mod::derive(syn::parse(item).unwrap()))
        }
    }
}

derive!("name": Name, derive_name -> name);
derive!("from_str": FromStr, derive_from_str -> from_str);
derive!("iter": Iter, derive_iter -> iter);
