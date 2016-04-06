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

use super::super::tree::*;
use super::component::*;
use super::immediate_publisher::*;
use super::subscriptionmanager::*;

///
/// A tree change bus queues up published changes until they are ready to send
///
pub struct TreeChangeBus {
    /// Changes that are waiting to be published
    waiting: Rc<RefCell<WaitingChanges>>,

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
    waiting: Rc<RefCell<WaitingChanges>>
}

impl TreeChangeBus {
    ///
    /// Creates a new bus publisher
    ///
    pub fn new() -> TreeChangeBus {
        TreeChangeBus { 
            waiting:        Rc::new(RefCell::new(WaitingChanges { waiting: vec![] })),
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
