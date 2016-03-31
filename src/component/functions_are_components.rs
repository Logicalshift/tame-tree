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
impl BoxedComponentFactory for Box<Fn(&TreeChange) -> TreeChange> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn create_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
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
/// Provides a component function that 
///
impl BoxedComponentFactory for Box<Fn(&TreeRef) -> TreeRef> {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn create_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
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
