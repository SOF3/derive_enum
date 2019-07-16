// derive_enum
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

#[derive(Debug, Default, PartialEq)]
struct Grault {
    f: f32,
}

#[derive(Debug, derive_enum::Iter, PartialEq)]
enum Foo {
    Bar,
    Qux(i32),
    Corge { st: String, grault: Grault },
}

#[test]
fn test() {
    use derive_enum::Iter;

    let iter = Foo::all();
    let vec = iter.map(|(name, f)| (name, f())).collect::<Vec<_>>();
    assert_eq!(vec, vec![
               ("Bar", Foo::Bar),
               ("Qux", Foo::Qux(0i32)),
               ("Corge", Foo::Corge { st: Default::default(), grault: Default::default() }),
    ]);
}
