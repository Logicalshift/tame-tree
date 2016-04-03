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
//! ```
//! # use tametree::component::*;
//! let add_one = component_fn(|x: &i32| { x+1 });
//! let add_two = component_fn(|x: &i32| { x+2 });
//! 
//! let add_three = Pipe(add_one, add_two);
//! # 
//! # let mut endpoint = ComponentEndPoint::<i32, i32>::new(add_three);
//! # endpoint.send(1);
//! # assert!(endpoint.recv().unwrap() == 4);
//! ```
//!
//! This pipe can be used as a component:
//!
//! ```
//! # use tametree::component::*;
//! # let add_one = component_fn(|x: &i32| { x+1 });
//! # let add_two = component_fn(|x: &i32| { x+2 });
//! # 
//! # let add_three = Pipe(add_one, add_two);
//! #
//! let mut endpoint = ComponentEndPoint::<i32, i32>::new(add_three);
//! endpoint.send(1);
//! assert!(endpoint.recv().unwrap() == 4);
//! ```
//!
//!

use std::rc::*;

use super::component::*;
use super::immediate_publisher::*;

struct Pipeline(ComponentRef, ComponentRef);
impl Component for Pipeline { }
impl Drop for Pipeline { fn drop(&mut self) { } }

///
/// A component that takes the output of `TFirst` and connects it to the input of `TSecond`
///
pub struct Pipe<TFirst: ConvertToComponent, TSecond: ConvertToComponent>(pub TFirst, pub TSecond);

impl<TFirst: ConvertToComponent, TSecond: ConvertToComponent> ConvertToComponent 
for Pipe<TFirst, TSecond> {
    #[inline]
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let Pipe(first, second) = self;
        let pipeline_start      = ImmediatePublisher::new();
        let pipeline_end        = pipeline_start.create_consumer();

        let first_component     = first.into_component(consumer, pipeline_start);
        let second_component    = second.into_component(pipeline_end, publisher);

        Rc::new(Pipeline(first_component, second_component))
    }
}

/*
 * TODO: would like to do this for function components as it's more efficient
 * but figuring out how to write the types so we don't get a conflict with the more generic version isn't easy
 *
 * use rustc_serialize::*;
impl<TIn: 'static + DecodeFromTreeNode, TResult: Decodable + Encodable + EncodeToTreeNode + 'static, TOut: 'static + ToTreeNode> ConvertToComponent 
for Pipe<Box<Fn(&TIn) -> TResult>, Box<Fn(&TResult) -> TOut>> {
    ...
}
*/
