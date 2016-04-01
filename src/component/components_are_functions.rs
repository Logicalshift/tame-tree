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
//! # Components are functions
//!
//! We can treat a component as a pair of functions: one to send data to it, and one to retrieve its
//! current status. This can be a convenient way to try them out.
//!
//! Components are asynchronous: they can receive or publish new results at any time. To accomodate this,
//! when a component is converted into a functional form, two functions result: `send()` to send data to
//! the component and `recv()` to retrieve the component's current state.
//!
//! # Example
//!
//! Here's the definition of a component that adds two numbers together:
//!
//! ```
//! # extern crate tametree;
//! # extern crate rustc_serialize;
//! # fn main() {
//! # use tametree::component::*;
//! # use tametree::component::immediate_publisher::*;
//! #
//! #[derive(RustcEncodable, RustcDecodable)]
//! struct InputTree {
//!     a: i32,
//!     b: i32,
//! };
//! impl EncodeToTreeNode for InputTree { }
//!
//! #[derive(RustcEncodable, RustcDecodable)]
//! struct ResultTree {
//!     result: i32
//! };
//! impl EncodeToTreeNode for ResultTree { }
//!
//! let component = component_fn(|input: &InputTree| { 
//!    ResultTree { result: input.a + input.b } 
//! });
//! # }
//! ```
//!
//! We can call `ComponentEndPoint::<InputTree, ResultTree>::new(component)` to turn this into
//! a pair of functions that are convenient to call:
//!
//! ```
//! # extern crate tametree;
//! # extern crate rustc_serialize;
//! # fn main() {
//! # use tametree::component::*;
//! # use tametree::component::immediate_publisher::*;
//! #
//! # #[derive(RustcEncodable, RustcDecodable)]
//! # struct InputTree {
//! #     a: i32,
//! #     b: i32,
//! # };
//! # impl EncodeToTreeNode for InputTree { }
//! # 
//! # #[derive(RustcEncodable, RustcDecodable)]
//! # struct ResultTree {
//! #     result: i32
//! # };
//! # impl EncodeToTreeNode for ResultTree { }
//! # 
//! # let component = component_fn(|input: &InputTree| { 
//! #    ResultTree { result: input.a + input.b } 
//! # });
//! #
//! let mut endpoint = ComponentEndPoint::<InputTree, ResultTree>::new(component);
//!
//! endpoint.send(InputTree { a: 4, b: 7 });
//! let result_tree = endpoint.recv();  // == Some({ result: 11 }) as this component updates immediately
//! # assert!(result_tree.unwrap().result == 11);
//! # }
//!
//! ```

use std::marker::PhantomData;

use super::super::tree::*;
use super::component::*;
use super::immediate_publisher::*;
use super::output_tree_publisher::*;

///
/// A component endpoint provides a basic input/output interface to a component, allowing data to be sent to it
/// and its output retrieved.
///
pub struct ComponentEndPoint<TIn, TOut>
    where   TIn: 'static + ToTreeNode,
            TOut: 'static + DecodeFromTreeNode {
    _component: ComponentRef,
    reader:     Box<Fn() -> TreeRef>,
    input:      PublisherRef,

    phantom_in: PhantomData<TIn>,
    phantom_out: PhantomData<TOut>
}

impl<TIn, TOut> ComponentEndPoint<TIn, TOut>
    where   TIn: 'static + ToTreeNode,
            TOut: 'static + DecodeFromTreeNode {
    ///
    /// Creates a new endpoint from an object that can create a component
    ///
    pub fn new<TComponent: ConvertToComponent>(component: TComponent) -> ComponentEndPoint<TIn, TOut> {
        let input       = ImmediatePublisher::new();
        let consumer    = input.create_consumer();
        let output      = OutputTreePublisher::new();
        let reader      = output.get_tree_reader();

        let component   = component.into_component(consumer, output);

        ComponentEndPoint { _component: component, reader: reader, input: input, phantom_in: PhantomData, phantom_out: PhantomData }
    }

    ///
    /// Sends new data to the component
    ///
    #[inline]
    pub fn send(&mut self, data: TIn) {
        self.input.publish(TreeChange::new(&(), TreeChangeType::Child, Some(&data.to_tree_node())));
    }

    ///
    /// Retrieves the current state of the component's output
    ///
    /// If the output does not conform to the type `TOut`, then this will return `None`
    ///
    #[inline]
    pub fn recv(&self) -> Option<TOut> {
        let reader = &self.reader;

        TOut::new_from_tree(&reader()).ok()
    }
}
