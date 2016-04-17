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
//! # Endpoints
//!
//! Endpoints can be used to retrieve both a sending and a receiving function for a component.
//!
//! ## Example
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
//! ```
//!
//! # Receiver functions
//!
//! This adds the ability to call get_receiver() with a type on any consumer in order to create a function
//! that can be called to retrieve its current value as a tree or any object that can be decoded from a
//! tree.
//!
//! ## Example
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
//! # let mut publisher         = ImmediatePublisher::new();
//! # let mut input_consumer    = publisher.create_consumer();
//! # let mut output_publisher  = ImmediatePublisher::new();
//! # let mut consumer          = output_publisher.create_consumer();
//! # let _active_component     = component.into_component(input_consumer, output_publisher);
//! let mut receiver: RecvFn<ResultTree> = consumer.get_receiver();
//!
//! publisher.publish(TreeChange::new(&(), &InputTree { a: 4, b: 7 }));
//! let result_tree = receiver();           // == Some(12) for our test component
//! # assert!(result_tree.unwrap().result == 11);
//! # }
//!
//! ```

use std::rc::*;
use std::marker::PhantomData;

use super::super::tree::*;
use super::super::util::clonecell::*;
use super::component::*;
use super::immediate_publisher::*;
use super::output_tree_publisher::*;

///
/// Defines the type of a receiver function
///
pub type RecvFn<TOut> = Box<Fn() -> Option<TOut>>;

///
/// Trait implemented by objects that can call a receiving function
///
pub trait Receiver<TOut> {
    ///
    /// Retrieves a function that can be used to get the last known value of this receiver (or `None` if it can't be converted to `TOut`)
    ///
    fn get_receiver(&mut self) -> RecvFn<TOut>;
}

impl Receiver<TreeRef> for ConsumerRef {
    ///
    /// Retrieves a function that can be used to get the last known value of this receiver (or `None` if it can't be converted to `TOut`)
    ///
    fn get_receiver(&mut self) -> RecvFn<TreeRef> {
        let tree        = Rc::new(CloneCell::new("".to_tree_node()));
        let also_tree   = tree.clone();

        self.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            let current_tree = (*tree).get();
            let altered_tree = change.apply(&current_tree);
            (*tree).set(altered_tree);
        }));

        Box::new(move || {
            Some((*also_tree).get())
        })
    }
}

impl<TOut: 'static + DecodeFromTreeNode + Sized> Receiver<TOut> for ConsumerRef {
    ///
    /// Retrieves a function that can be used to get the last known value of this receiver (or `None` if it can't be converted to `TOut`)
    ///
    fn get_receiver(&mut self) -> Box<Fn() -> Option<TOut>> {
        let tree_receiver: Box<Fn() -> Option<TreeRef>> = self.get_receiver();

        Box::new(move || {
            if let Some(tree) = tree_receiver() {
                if let Ok(result) = TOut::new_from_tree(&tree) {
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

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
        self.input.publish(TreeChange::new(&(), &data.to_tree_node()));
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
