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

//!
//! # Pipes
//!
//! Pipes should be familiar to anyone who has used a UNIX-like operating system before. They
//! connect the output of one component to the input of another. The tuple type `Pipe` is used
//! to represent the pipe between two components. For example:
//!
//! ```ignore
//! Pipe(first, second).into_component(consumer, publisher)
//! ```
//!

use super::super::tree::*;
use super::component::*;

///
/// Trait implemented by types that represent a pipe
///
pub trait ToPipe {
    ///
    /// The type of the output component
    ///
    type TOut: ConvertToComponent;

    ///
    /// Pipes a component into another component
    ///
    fn to_pipe(self) -> Self::TOut;
}

///
/// A component that takes the output of `TFirst` and connects it to the input of `TSecond`
///
pub struct Pipe<TFirst: ConvertToComponent, TSecond: ConvertToComponent>(TFirst, TSecond);

impl<TFirst: ConvertToComponent, TSecond: ConvertToComponent> ToPipe 
for Pipe<TFirst, TSecond> {
    type TOut = Box<Fn(&TreeChange) -> TreeChange>;

    fn to_pipe(self) -> Self::TOut {
        unimplemented!()
    }
}

impl<TFirst: ConvertToComponent, TSecond: ConvertToComponent> ConvertToComponent 
for Pipe<TFirst, TSecond> {
    #[inline]
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        self.to_pipe().into_component(consumer, publisher)
    }
}

/*
 * TODO: would like to do this for function components as it's more efficient
 * but figuring out how to write the types so we don't get a conflict with the more generic version isn't easy
 *
 * use rustc_serialize::*;
impl<TIn: 'static + DecodeFromTreeNode, TResult: Decodable + Encodable + EncodeToTreeNode + 'static, TOut: 'static + ToTreeNode> ToPipe 
for Pipe<Box<Fn(&TIn) -> TResult>, Box<Fn(&TResult) -> TOut>> {
    type TOut = Box<Fn(&TIn) -> TOut>;

    fn to_pipe(self) -> Self::TOut {
        Box::new(move |input| {
            let intermediate = self.0(input);
            self.1(&intermediate)
        })
    }
}
*/
