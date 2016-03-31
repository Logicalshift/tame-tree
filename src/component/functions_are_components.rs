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

impl FunctionComponent {
    fn register<F: Fn(&TreeChange) -> TreeChange + 'static>(&self, consumer: ConsumerRef, publisher: PublisherRef, func: F) {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let mut action          = Box::new(func);

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            let change_result = action(change);
            our_publisher.publish(change_result);
        }));
    }
}

impl ComponentFactory for Fn(&TreeChange) -> TreeChange + 'static {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn create(&self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        let mut our_consumer    = consumer;
        let mut our_publisher   = publisher;
        let action              = Box::new(self.clone());

        our_consumer.subscribe(TreeAddress::Here, TreeExtent::SubTree, Box::new(move |change| {
            //let change_result = action(change);
            //our_publisher.publish(change_result);
        }));

        return Rc::new(FunctionComponent);
    }
}