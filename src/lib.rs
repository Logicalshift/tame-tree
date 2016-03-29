//
//   Copyright 2016 Andrew Hunter
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
//

//! # TreeSync
//! 
//! TreeSync is a library for synchronising trees and reacting to change.
//!
//! ## Philosophy
//!
//! Software is hard to grow; TreeSync is designed as a tool to make it easier to build large pieces of software
//! from smaller ones. This is done by building software as components that use this library for communication.
//!
//! The hypothesis is this: software is easier to work with when it's small: it's easier to reason about and 
//! easier to test. However, real world requirements pushes software to become ever larger, and this inevitably
//! results in problems of various kinds. Discipline can stave these problems off but inevitably deadlines and
//! other pressures causes cracks in the facade as in a conflict between the needs of the software developer and
//! the needs of a business, the business needs will always end up taking priority.
//!
//! If smallness and isolation is inherent to the architecture of a piece of software, then this conflict will
//! have less effect: small pieces of software are easier to reason about and isolated software can be replaced
//! at a time that is convenient to both the business and the developer. This seems hard to do when the 
//! requirements for modern software often involves a large number of interrelated features.
//! 
//! TreeSync provides a way to split the design of software into two: a set of components with a narrow, isolated
//! view of the world, and a system layer that connects the components together into a larger whole. Software is
//! expanded by writing more components and tying them together at the system layer rather than by growing a single
//! monolithic whole. Components can be quickly replaced due to their limited dependencies: they don't even need to
//! be kept small as they can be divided later on if needed.
//!
//! This is intended to have some parallels with the design of UNIX, which is also designed around small single
//! purpose components, a feature that has contributed to its longevity by making it possible for it to be 
//! continuously modernised and adapted to new tasks without requiring a full rewrite.
//!
//! Where UNIX uses streams for communication, TreeSync instead uses trees to better support interactive applications.

extern crate rustc_serialize;

pub mod tree;
pub mod component;
mod util;
