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
    fn name(&self) -> &'static str;
}

#[cfg(feature = "from_str")] pub use macros::FromStr;

#[derive(Debug)]
pub enum Error {
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
