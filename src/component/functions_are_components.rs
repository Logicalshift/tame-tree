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

///
/// Makes a function into a variant that can be used with a suitable `into_component()` call.
///
/// Short for 'make component'
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
/// let component = mk_com(|tree: &TreeRef| { tree.clone() }).into_component(consumer, publisher);
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
/// To make this less of a nightmare to use, the mk_com function tells rust that a function can be boxed and
/// helps out by inferring the various parameters.
///
#[inline]
pub fn mk_com<TIn, TOut, F>(func: F) -> Box<Fn(&TIn) -> TOut> where F: Fn(&TIn) -> TOut + 'static {
    Box::new(func)
}

#[cfg(test)]
mod component_function_tests {
    use super::super::super::component::*;
    use super::super::immediate_publisher::*;
    use super::super::output_tree_publisher::*;

    #[test]
    pub fn can_create_tree_change_component() {
        let mut input_publisher = ImmediatePublisher::new();
        let consumer            = input_publisher.create_consumer();

        let output_publisher    = OutputTreePublisher::new();
        let result_reader       = output_publisher.get_tree_reader();
        
        let _component = mk_com(|_change: &TreeChange| {
            TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"passed".to_tree_node())) 
        }).into_component(consumer, output_publisher);

        // Publish something to our function
        input_publisher.publish(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"test".to_tree_node())));

        // Check that the output was 'passed'
        let result = result_reader();
        assert!(result.get_tag() == "passed")
    }
}
