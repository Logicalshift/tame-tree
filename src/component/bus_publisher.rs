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
//! # The bus publisher
//!
//! This publisher works by queueing up changes and then sending them as batches. It is
//! useful for cases where there may be feedback loops or where a single change can have
//! a large list of consequences.
//!
//! Bus publishers can also support multiple input sources: changes are published in the 
//! order that they arrive. This can be used to aggregate the output of several components
//! into a single tree.
//!

use std::rc::*;
use std::cell::*;
use std::mem;

use super::super::tree::*;
use super::component::*;
use super::subscriptionmanager::*;

///
/// A tree change bus queues up published changes until they are ready to send
///
pub struct TreeChangeBus {
    /// Changes that are waiting to be published
    /// (Rc so we can share between publishers, RefCell so we can update, Box so we can swap)
    waiting: Rc<RefCell<Box<WaitingChanges>>>,

    /// Consumers of this publisher
    subscriptions: Rc<SubscriptionManager<ConsumerRegistration>>
}

///
/// Changes waiting to be sent
///
struct WaitingChanges {
    waiting: Vec<Box<TreeChange>>
}

///
/// Stores a registration of a consumer
///
#[derive(Clone)]
struct ConsumerRegistration {
    address: TreeAddress,
    extent: TreeExtent
}

///
/// A consumer that receives changes from a TreeChangeBus
///
struct BusConsumer {
    subscriptions: Rc<SubscriptionManager<ConsumerRegistration>>
}

///
/// A publisher that sends changes to a TreeChangeBus
///
struct BusPublisher {
    /// Changes that are waiting to be published
    waiting: Rc<RefCell<Box<WaitingChanges>>>
}

impl TreeChangeBus {
    ///
    /// Creates a new bus publisher
    ///
    pub fn new() -> TreeChangeBus {
        TreeChangeBus { 
            waiting:        Rc::new(RefCell::new(Box::new(WaitingChanges { waiting: vec![] }))),
            subscriptions:  Rc::new(SubscriptionManager::new())
        }
    }

    ///
    /// Creates a publisher that will send notifications to this object
    ///
    pub fn create_publisher(&self) -> PublisherRef {
        Box::new(BusPublisher { waiting: self.waiting.to_owned() })
    }

    ///
    /// Creates a consumer that will receive notifications from this publisher
    ///
    pub fn create_consumer(&self) -> ConsumerRef {
        Box::new(BusConsumer { subscriptions: self.subscriptions.clone() })
    }

    ///
    /// Pumps any published messages to the consumer
    ///
    pub fn pump(&mut self) {
        // Create a new list of waiting items and swap it for the active list
        let to_send = {
            let mut borrowed_waiting    = self.waiting.borrow_mut();
            let mut current_value       = Box::new(WaitingChanges { waiting: vec![] });

            mem::swap(&mut *borrowed_waiting, &mut current_value);

            current_value
        };

        // Publish the items in to_send
        for change in to_send.waiting {
            self.subscriptions.call_subscriptions(&|registration| {
                change.applies_to(&registration.address, &registration.extent).unwrap_or(false)
            }, &change);
        }
    }

    ///
    /// Pumps published messages to the consumer repeatedly until there are none left to process
    ///
    pub fn flush(&mut self) {
        // Pump published messages until no more are generated
        loop {
            if self.waiting.borrow().waiting.len() <= 0 {
                return;
            }

            self.pump();
        }
    }
}

impl Publisher for BusPublisher {
    ///
    /// Publishes a change to the consumers of this component
    ///
    #[inline]
    fn publish(&mut self, change: TreeChange) {
        self.waiting.borrow_mut().waiting.push(Box::new(change))
    }
}

impl Consumer for BusConsumer {
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

#[cfg(test)]
mod bus_publisher_tests {
    use super::super::super::component::*;
    use super::super::output_tree_publisher::*;
    use super::*;

    #[test]
    pub fn can_pump_bus() {
        let mut input_bus           = TreeChangeBus::new();
        let mut input_publisher     = input_bus.create_publisher();
        let output_publisher        = OutputTreePublisher::new();
        let input_consumer          = input_bus.create_consumer();
        let output_reader           = output_publisher.get_tree_reader();
        let add_one                 = component_fn(|x: &i32| { x+1 });

        let _add_component          = add_one.into_component(input_consumer, output_publisher);

        input_publisher.publish(TreeChange::new(&(), &1));
        input_bus.pump();
        let output = output_reader();
        assert!(output.get_value().to_int(0) == 2);
    }

    #[test]
    pub fn can_have_feedback() {
        let mut input_bus           = TreeChangeBus::new();
        let mut input_publisher     = input_bus.create_publisher();
        let mut feedback_publisher  = input_bus.create_publisher();
        let output_publisher        = OutputTreePublisher::new();
        let input_consumer          = input_bus.create_consumer();
        let output_reader           = output_publisher.get_tree_reader();

        // Feedback component that sends a message back to itself to reduce the value if it's greater than 0
        let tend_to_zero            = component_fn_mut(move |x: &i32| { 
            if *x > 0 {
                feedback_publisher.publish(TreeChange::new(&(), &(x-1)));
            }
            *x
        });

        let _becomes_zero_component = tend_to_zero.into_component(input_consumer, output_publisher);

        input_publisher.publish(TreeChange::new(&(), &10));
        input_bus.pump();
        assert!(output_reader().get_value().to_int(0) == 10);

        input_bus.pump();
        assert!(output_reader().get_value().to_int(0) == 9);

        input_bus.pump();
        assert!(output_reader().get_value().to_int(0) == 8);

        input_bus.flush();
        assert!(output_reader().get_value().to_int(0) == 0);
    }
}
