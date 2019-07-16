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

macro_rules! decl_derive {
    ($feature: literal: $name: ident, $fn: ident -> $mod: ident | $doc: literal) => {
        #[cfg(feature = $feature)]
        mod $mod;
        
        #[cfg(feature = $feature)]
        #[proc_macro_derive($name)]
        #[doc = $doc]
        pub fn $fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            From::from($mod::derive(syn::parse(item).unwrap()))
        }
    }
}

decl_derive!("name": Name, derive_name -> name | "
        Provides the enum variant name given the instance.
        
        Requires the <em>name</em> feature.
    ");


decl_derive!("from_str": FromStr, derive_from_str -> from_str | "
        Implements the FromStr trait for an enum.

        The given string must be exactly the enum variant name, without parentheses, braces or
        otherwise information on the inner fields.

        A new instance of the corresponding enum variant is allocated.
        If there are inner fields, they must all implement `Default`, and the default value of the type
        would be returned.

        Requires the <em>from_str</em> feature.
    ");


decl_derive!("iter": Iter, derive_iter -> iter | "
        Implements the Iter trait for an enum.

        Requires the <em>iter</em> feature.
    ");
