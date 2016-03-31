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
impl BoxedConvertToComponent for Box<Fn(&TreeChange) -> TreeChange> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
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
impl BoxedConvertToComponent for Box<FnMut(&TreeChange) -> TreeChange> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
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
impl BoxedConvertToComponent for Box<Fn(&TreeRef) -> TreeRef> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
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
impl BoxedConvertToComponent for Box<FnMut(&TreeRef) -> TreeRef> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn into_component_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
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

#[cfg(test)]
mod component_function_tests {
    use super::super::super::component::*;
    use super::super::immediate_publisher::*;
    use super::super::output_tree_publisher::*;

    fn make_tree_fn<F: Fn(&TreeChange) -> TreeChange + 'static>(func: F) -> Box<Fn(&TreeChange) -> TreeChange> {
        Box::new(func)
    }

    #[test]
    pub fn can_create_tree_change_component() {
        let mut publisher       = ImmediatePublisher::new();
        let consumer            = publisher.create_consumer();
        let output_publisher    = OutputTreePublisher::new();
        let result_reader       = output_publisher.get_tree_reader();
        
        // TODO: rust isn't smart enough to realise it can coerce a closure into a Fn, so this is awkward
        // TODO: addtionally let foo: Fn() = | { } is a compilation error just to make things even harder (even though we can use it in a parameter)
        // Using a helper function here cleans up the code a bit; annoyingly we have to specify the function type in the name
        let component_fn = make_tree_fn(|_change: &TreeChange| -> TreeChange {
            TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"passed".to_tree_node())) 
        });

        let _component = component_fn.into_component_boxed(consumer, output_publisher);

        // Publish something to our function
        publisher.publish(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"test".to_tree_node())));

        // Check that the output was 'passed'
        let result = result_reader();
        assert!(result.get_tag() == "passed")
    }
}
