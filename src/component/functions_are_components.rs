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
//! # Functions are components
//!
//! A component can be considered to be simply a transformation from one tree to another, or alternatively
//! a routine that reacts to a change in an input tree with a change to the output tree.
//!
//! Any function that takes a single parameter representing a type that can be derived from a tree and returns
//! a value that can be converted into a tree can be used as a component function. The method `component_fn()`
//! will turn the type into a `ConvertToComponent` object (useful when composing components), and the method
//! `to_component()` will create a component directly from a function.
//!
//! Example:
//!
//! ```
//! # extern crate tametree;
//! # extern crate rustc_serialize;
//! # fn main() {
//! # use tametree::component::*;
//! # use tametree::component::immediate_publisher::*;
//! #
//! # let input_publisher   = ImmediatePublisher::new();
//! # let consumer          = input_publisher.create_consumer();
//! # let publisher         = ImmediatePublisher::new();
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
//! let component = to_component(consumer, publisher, |input: &InputTree| { 
//!    ResultTree { result: input.a + input.b } 
//! });
//! # }
//!
//! ```
//!
//! Alternatively, a component could just respond directly to tree changes:
//!
//! ```
//! # use tametree::component::*;
//! # use tametree::component::immediate_publisher::*;
//! #
//! # let input_publisher   = ImmediatePublisher::new();
//! # let consumer          = input_publisher.create_consumer();
//! # let publisher         = ImmediatePublisher::new();
//! let component = to_component(consumer, publisher, |_change: &TreeChange| { 
//!    TreeChange::new(&(), TreeChangeType::Child, None::<&TreeRef>)
//! });
//! ```
//!
//! Responding directly to tree changes is useful when a component doesn't want to keep an entire tree in 
//! memory: an example of where this is useful is when making a subtree act like a stream.
//!

use std::rc::*;

use super::component::*;
use super::super::tree::*;

struct FunctionComponent;

impl Component for FunctionComponent {
}

impl Drop for FunctionComponent {
    fn drop(&mut self) {
    }
}

///
/// Simplest form of 'component function': a function that receives a `TreeChange` indicating how the
/// input tree has changed, and returns a new change indicating how the output has changed.
///
impl ConvertToComponent for Box<Fn(&TreeChange) -> TreeChange> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let action              = self;

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            let change_result = action(change);
            our_publisher.publish(change_result);
        }));

        return Rc::new(FunctionComponent);
    }
}

///
/// Simplest form of 'component function': a function that receives a `TreeChange` indicating how the
/// input tree has changed, and returns a new change indicating how the output has changed.
///
/// This variant allows for mutable state.
///
impl ConvertToComponent for Box<FnMut(&TreeChange) -> TreeChange> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let mut action          = self;

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            let change_result = action(change);
            our_publisher.publish(change_result);
        }));

        return Rc::new(FunctionComponent);
    }
}

///
/// Provides a component function that converts an input tree to an output tree
///
impl ConvertToComponent for Box<Fn(&TreeRef) -> TreeRef> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let action              = self;

        let mut tree = "empty".to_tree_node();

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            tree = change.apply(&tree);

            let new_tree = action(&tree);

            our_publisher.publish(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&new_tree)));
        }));

        return Rc::new(FunctionComponent);
    }
}

///
/// Provides a component function that converts an input tree to an output tree
///
/// This variant allows for mutable state
///
impl ConvertToComponent for Box<FnMut(&TreeRef) -> TreeRef> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let mut action          = self;

        let mut tree = "empty".to_tree_node();

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            tree = change.apply(&tree);

            let new_tree = action(&tree);

            our_publisher.publish(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&new_tree)));
        }));

        return Rc::new(FunctionComponent);
    }
}

impl<TIn: 'static + DecodeFromTreeNode, TOut: 'static + ToTreeNode> ConvertToComponent for Box<FnMut(&TIn) -> TOut> {
    ///
    /// Creates a component that consumes from a tree and pub
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let mut action          = self;

        let mut tree = "empty".to_tree_node();

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            tree = change.apply(&tree);

            // TODO: once we have error handling, deal with decoding failing here
            let decoded_or_err  = TIn::new_from_tree(&tree);
            if let Ok(decoded) = decoded_or_err {
                let new_object  = action(&decoded);
                let new_tree    = new_object.to_tree_node();

                our_publisher.publish(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&new_tree)));
            }
        }));

        return Rc::new(FunctionComponent);
    }
}

impl<TIn: 'static + DecodeFromTreeNode, TOut: 'static + ToTreeNode> ConvertToComponent for Box<Fn(&TIn) -> TOut> {
    ///
    /// Creates a component that consumes from a tree and pub
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let action = self;

        component_fn_mut(move |val| { action(val) }).into_component(consumer, publisher)
    }
}

///
/// Makes a function into a variant that can be used with a suitable `into_component()` call.
///
/// For example:
///
/// ```
/// # use tametree::component::*;
/// # use tametree::component::immediate_publisher::*;
/// #
/// # let input_publisher   = ImmediatePublisher::new();
/// # let consumer          = input_publisher.create_consumer();
/// # let publisher         = ImmediatePublisher::new();
/// let component = component_fn(|tree: &TreeRef| { tree.clone() }).into_component(consumer, publisher);
/// ```
///
/// This exists to get around some limitations in rust's type inference.
///
/// What would be neat is if we could do `(|change: &TreeRef| -> TreeRef { ... }).into_component()`
/// and have rust figure out that there's an implementation on a `Fn(&TreeRef) -> TreeRef` that we
/// can use to generate the result. Even `Box::new(...).into_component()` would be OK.
///
/// However, closures and Fns are different types and don't get coerced implicitly so that doesn't work.
/// That is, `Box::new(...).into_component()` will produce an error as the box has the closure type and rust
/// isn't smart enough to realise there's a cast to `Box<Fn()>`
///
/// Here's what you have to do as a result:
///
/// ```
/// # use tametree::component::*;
/// # use tametree::component::immediate_publisher::*;
/// #
/// # let input_publisher   = ImmediatePublisher::new();
/// # let consumer          = input_publisher.create_consumer();
/// # let publisher         = ImmediatePublisher::new();
/// let component_fn: Box<Fn(&TreeRef) -> TreeRef> = Box::new(|tree: &TreeRef| { tree.clone() });
/// let component = component_fn.into_component(consumer, publisher);
/// ```
///
/// Type inference: utterly defeated. (`Box::<Fn blah blah>::new` doesn't work either because it coerces the
/// closure to the Fn trait too early and thus produces an error)
///
/// To make this less of a nightmare to use, the component_fn function tells rust that a function can be boxed and
/// helps out by inferring the various parameters.
///
#[inline]
pub fn component_fn<TIn, TOut, F>(func: F) -> Box<Fn(&TIn) -> TOut> where F: Fn(&TIn) -> TOut + 'static {
    Box::new(func)
}

///
/// Version of component_fn that works on `FnMut` functions
///
/// For example:
///
/// ```
/// # use tametree::component::*;
/// # use tametree::component::immediate_publisher::*;
/// #
/// # let input_publisher   = ImmediatePublisher::new();
/// # let consumer          = input_publisher.create_consumer();
/// # let publisher         = ImmediatePublisher::new();
/// let mut times_run       = 0;
/// let component = component_fn_mut(move |tree: &TreeRef| { 
///     times_run = times_run + 1;
///     tree.clone() 
/// }).into_component(consumer, publisher);
/// ```
///
#[inline]
pub fn component_fn_mut<TIn, TOut, F>(func: F) -> Box<FnMut(&TIn) -> TOut> where F: FnMut(&TIn) -> TOut + 'static {
    Box::new(func)
}

///
/// Starts running a function as a component
///
/// # Example
///
/// ```
/// # use tametree::component::*;
/// # use tametree::component::immediate_publisher::*;
/// #
/// # let input_publisher   = ImmediatePublisher::new();
/// # let consumer          = input_publisher.create_consumer();
/// # let publisher         = ImmediatePublisher::new();
/// let pass_through_component = to_component(consumer, publisher, |tree: &TreeRef| { tree.clone() });
/// ```
///
#[inline]
pub fn to_component<TIn, TOut, F>(consumer: ConsumerRef, publisher: PublisherRef, func: F) -> ComponentRef 
    where   F: Fn(&TIn) -> TOut + 'static, 
            Box<Fn(&TIn) -> TOut> : ConvertToComponent {
    component_fn(func).into_component(consumer, publisher)
}


///
/// Starts running a mutable function as a component
///
/// # Example
///
/// ```
/// # use tametree::component::*;
/// # use tametree::component::immediate_publisher::*;
/// #
/// # let input_publisher   = ImmediatePublisher::new();
/// # let consumer          = input_publisher.create_consumer();
/// # let publisher         = ImmediatePublisher::new();
/// let mut times_run       = 0;
/// let pass_through_component = to_component_mut(consumer, publisher, move |tree: &TreeRef| { 
///     times_run = times_run + 1; 
///     tree.clone() 
/// });
/// ```
///
#[inline]
pub fn to_component_mut<TIn, TOut, F>(consumer: ConsumerRef, publisher: PublisherRef, func: F) -> ComponentRef 
    where   F: FnMut(&TIn) -> TOut + 'static, 
            Box<FnMut(&TIn) -> TOut> : ConvertToComponent {
    component_fn_mut(func).into_component(consumer, publisher)
}

#[cfg(test)]
mod component_function_tests {
    use rustc_serialize::*;

    use super::super::super::component::*;
    use super::super::immediate_publisher::*;
    use super::super::output_tree_publisher::*;

    #[test]
    pub fn can_create_tree_change_component() {
        let mut input_publisher = ImmediatePublisher::new();
        let consumer            = input_publisher.create_consumer();

        let output_publisher    = OutputTreePublisher::new();
        let result_reader       = output_publisher.get_tree_reader();
        
        let _component = to_component(consumer, output_publisher, |_change: &TreeChange| {
            TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"passed".to_tree_node())) 
        });

        // Publish something to our function
        input_publisher.publish(TreeChange::new(&(), TreeChangeType::Child, Some(&"test".to_tree_node())));

        // Check that the output was 'passed'
        let result = result_reader();
        assert!(result.get_tag() == "passed")
    }

    #[test]
    pub fn can_create_tree_ref_component() {
        let mut input_publisher = ImmediatePublisher::new();
        let consumer            = input_publisher.create_consumer();

        let output_publisher    = OutputTreePublisher::new();
        let result_reader       = output_publisher.get_tree_reader();
        
        let _component = to_component(consumer, output_publisher, |new_tree: &TreeRef| {
            new_tree.clone()
        });

        // Publish something to our function
        input_publisher.publish(TreeChange::new(&(), TreeChangeType::Child, Some(&"passed".to_tree_node())));

        // Check that the output was 'passed'
        let result = result_reader();
        assert!(result.get_tag() == "passed")
    }

    #[test]
    pub fn can_create_encoding_decoding_component() {
        let mut input_publisher = ImmediatePublisher::new();
        let consumer            = input_publisher.create_consumer();

        let output_publisher    = OutputTreePublisher::new();
        let result_reader       = output_publisher.get_tree_reader();
        
        #[derive(RustcEncodable, RustcDecodable)]
        struct InputTree {
            a: i32,
            b: i32,
        };
        impl EncodeToTreeNode for InputTree { }
        
        #[derive(RustcEncodable, RustcDecodable)]
        struct ResultTree {
            result: i32
        };
        impl EncodeToTreeNode for ResultTree { }
        
        let _component = to_component(consumer, output_publisher, |input: &InputTree| {
            ResultTree { result: input.a + input.b } 
        });

        // Publish something to our function
        input_publisher.publish(TreeChange::new(&(), TreeChangeType::Child, Some(&InputTree { a: 1, b: 2 }.to_tree_node())));

        // Check that the output was 'passed'
        let result = result_reader();
        assert!(result.get_child_ref_at("result").unwrap().get_value().to_int(0) == 3)
    }
}
