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

use super::super::tree::*;

use super::component::*;
use super::subscriptionmanager::*;

///
/// Stores a registration of a consumer
///
#[derive(Clone)]
struct ConsumerRegistration {
    address: TreeAddress,
    extent: TreeExtent
}

///
/// Consumer for data written by an immediate publisher
///
struct ImmediateConsumer {
    ///
    /// Where subscriptions can be registered for this consumer
    ///
    subscriptions: Rc<SubscriptionManager<ConsumerRegistration>>
}

impl Consumer for ImmediateConsumer {
    ///
    /// Calls a function whenever a particular section of the tree has changed
    ///
    fn subscribe(&mut self, address: TreeAddress, extent: TreeExtent, callback: ConsumerCallback) {
        // Need to persuade rust that it can call the FnMut (assign parameter to a mutable variable)
        let mut also_callback = callback;

        self.subscriptions.add_subscription(ConsumerRegistration { address: address.clone(), extent: extent }, Box::new(move |change| {
            // The change we get from the subscription will have an address relative to the root of the tree
            // Make the subscription change relative to the address that was subscribed to 
            let maybe_relative_change = change.relative_to(&address);
            if let Some(relative_change) = maybe_relative_change {
                also_callback(&relative_change);
            }
        }));
    }
}

///
/// Publisher that immediately sends changes to its consumers. Can be used as a consumer factory.
///
pub struct ImmediatePublisher {
    ///
    /// Subscriptions for this publisher
    ///
    subscriptions: Rc<SubscriptionManager<ConsumerRegistration>>
}

impl ImmediatePublisher {
    ///
    /// Creates a new immediate publisher
    ///
    pub fn new() -> Box<ImmediatePublisher> {
        Box::new(ImmediatePublisher { subscriptions: Rc::new(SubscriptionManager::new()) })
    }

    ///
    /// Creates a consumer that will receive notifications from this publisher
    ///
    pub fn create_consumer(&self) -> ConsumerRef {
        Box::new(ImmediateConsumer { subscriptions: self.subscriptions.clone() })
    }
}

impl Publisher for ImmediatePublisher {
    ///
    /// Publishes a change to the consumers of this component
    ///
    fn publish(&mut self, change: TreeChange) {
        self.subscriptions.call_subscriptions(&|registration| {
            change.applies_to(&registration.address, &registration.extent).unwrap_or(false)
        }, &change);
    }
}

#[cfg(test)]
mod immediate_publisher_tests {
    use std::cell::*;
    use std::rc::*;

    use super::super::super::component::*;
    use super::*;

    #[test]
    fn can_consume_published_item() {
        let mut publisher   = ImmediatePublisher::new();
        let mut consumer    = publisher.create_consumer();

        let our_count       = Rc::new(Cell::new(0));
        let their_count     = our_count.clone();

        let consumer_tree: RefCell<TreeRef> = RefCell::new(Rc::new("empty".to_tree_node()));

        // Subscribe to the second child of the root
        // Ie, tree should look like this:
        //
        // root
        //   +- Anything
        //   +- ConsumerTree
        //        +- add(value)
        consumer.subscribe(1.to_tree_address(), TreeExtent::SubTree, Box::new(move |change| {
            // Update the tree
            let mut tree = consumer_tree.borrow_mut();
            let new_tree = change.apply(&tree.clone());
            *tree = new_tree;

            // Tree can have an 'add' node that specifies how much to add to the count for this change
            let old_val     = their_count.get();
            let tree_value  = tree.get_child_ref_at("add").map(|val| { val.get_value().to_int(0) }).unwrap_or(0);

            their_count.set(old_val + tree_value);
        }));

        // count is initially 0
        assert!(our_count.get() == 0);

        // Publish an add of 1 to set the count to 1
        let just_add = ("add", 1).to_tree_node();

        // Change the child of the second child of the root to the 'just_add' tree
        let modify_child_of_subscribed_tree = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, Some(&just_add));
        assert!(!modify_child_of_subscribed_tree.relative_to(&1.to_tree_address()).is_none());
        publisher.publish(modify_child_of_subscribed_tree);

        assert!(our_count.get() == 1);

        // Publish an add of 1 to set the count to 1
        let whole_tree = tree!("root", "some_other_tree", tree!("consumer_target", ("add", 2)));

        // Replace the entire tree with the tree above
        let modify_entire_tree = TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&whole_tree));
        publisher.publish(modify_entire_tree);

        assert!(our_count.get() == 3);
    }
}
