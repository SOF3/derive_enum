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

use derive_enum_macros as macros;

#[cfg(feature = "name")] pub use macros::Name;

#[cfg(feature = "name")]
pub trait Name {
    /// Returns the name of the enum variant.
    ///
    /// The result is not affected by inner fields.
    fn name(&self) -> &'static str;
}


#[cfg(feature = "from_str")] pub use macros::FromStr;


#[cfg(feature = "iter")] pub use macros::Iter;

/// Provides an iterator on the variants of an enum.
///
/// The iterator yields a tuple containing the name of the variant and a closure to create a default
/// instance of the enum variant.
/// The behaviour of the closure is equivalent to calling `FromStr`.
///
/// Requires the <em>iter</em> feature.
#[cfg(feature = "iter")]
pub trait Iter {
    fn all() -> Box<dyn Iterator<Item = (&'static str, &'static dyn Fn() -> Self)>>;
}

/// The generic error type for the crate.
#[derive(Debug)]
pub enum Error {
    /// Returned when a nonexistent enum variant is requested.
    NoSuchEnum,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoSuchEnum => write!(f, "No such enum"),
        }
    }
}

impl std::error::Error for Error {}
