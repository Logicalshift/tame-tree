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
use std::ops::*;

use super::super::tree::*;

pub type PublisherRef = Box<Publisher>;
pub type ConsumerRef = Box<Consumer>;

///
/// A publisher reports changes to a tree
///
pub trait Publisher {
    ///
    /// Publishes a change to the consumers of this component
    ///
    fn publish(&mut self, change: TreeChange);
}

///
/// Type of a consumer callback function
///
pub type ConsumerCallback = Box<FnMut(&TreeChange) -> ()>;

///
/// A consumer subscribes to published changes to a tree
///
pub trait Consumer {
    ///
    /// Calls a function whenever a particular section of the tree has changed
    ///
    fn subscribe(&mut self, address: TreeAddress, extent: TreeExtent, callback: ConsumerCallback);
}

///
/// A component consumes a tree and publishes a tree. 
///
pub trait Component : Drop {
}

///
/// References to components are used to keep track of the components that are currently active
///
pub type ComponentRef = Rc<Component>;

///
/// A component subscribes consumes a tree and publishes a transformed version. Components are built from
/// a factory.
///
pub trait ComponentFactory {
    ///
    /// Creates a component that consumes from a particular tree and publishes to a different tree
    ///
    fn create(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef;
}
