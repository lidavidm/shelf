// Copyright 2020 David Li <li.davidm96@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

extern crate chrono;
extern crate git2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod common;
pub mod item;
pub mod save;
pub mod series;
pub mod shelf;

pub use crate::shelf::Shelf;

#[cfg(test)]
mod tests {}
