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
//! # Hub
//!
//! A hub provides a way to connect components into a tree. Hubs don't store changes, but can be used with things like
//! `ComponentEndPoint` to make component results user accessible.
//!

use super::super::tree::*;
use super::component::*;
use super::bus_publisher::*;
use super::immediate_publisher::*;

///
/// 
///
pub struct Hub {
    ///
    /// Passes changes between components
    ///
    bus: TreeChangeBus,

    ///
    /// Components attached to this hub
    ///
    components: Vec<ComponentRef>
}

impl Hub {
    ///
    /// Creates a new hub
    ///
    pub fn new() -> Hub {
        Hub { bus: TreeChangeBus::new(), components: vec![] }
    }

    ///
    /// Returns a consumer that will read from a particular address relative to this hub
    ///
    pub fn read_from<T: ToTreeAddress>(&mut self, address: &T) -> ConsumerRef {
        // TODO: smarter routing that doesn't respond to every single event
        // TODO: ensure we stop listening when the ConsumerRef is released

        // Create an immediate publisher to push changes to
        let mut publisher   = ImmediatePublisher::new();
        let consumer        = publisher.create_consumer();

        let target_address  = address.to_tree_address();

        // Push changes to the consumer when the bus changes
        self.bus.create_consumer().subscribe(target_address, TreeExtent::SubTree, Box::new(move |change| {
            publisher.publish(change.clone());
        }));

        consumer
    }

    ///
    /// Returns a publisher that will write to a particular address relative to this hub
    ///
    pub fn publish_to<T: ToTreeAddress>(&mut self, address: &T) -> PublisherRef {
        // We use an immediate publish to relay changes to the tree
        let publisher           = ImmediatePublisher::new();
        let mut consumer        = publisher.create_consumer();

        // Whenever the user publishes to the immediate publisher, generate a tree publish event
        let mut bus_publisher   = self.bus.create_publisher();
        let target_address      = address.to_tree_address();

        consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            let relative_change = change.relative_to(&target_address);

            if let Some(relative_change) = relative_change {
                bus_publisher.publish(relative_change);
            }
        }));

        publisher
    }

    ///
    /// Attaches a component that reads from a particular address and publishes its results to another
    ///
    pub fn add_component<TComponent: ConvertToComponent, TFrom: ToTreeAddress, TTo: ToTreeAddress>(&mut self, component: TComponent, read_from: &TFrom, publish_to: &TTo) {
        let consumer    = self.read_from(read_from);
        let publisher   = self.publish_to(publish_to);

        self.components.push(component.into_component(consumer, publisher));
    }

    ///
    /// Pumps any messages waiting for this hub
    ///
    #[inline]
    pub fn pump(&mut self) {
        self.bus.pump();
    }

    ///
    /// Processes messages for this hub until there are no more to be processed
    ///
    #[inline]
    pub fn flush(&mut self) {
        self.bus.flush();
    }
}
