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

///
/// 
///
pub struct Hub {
    ///
    /// Passes changes between components
    ///
    publisher: TreeChangeBus
}

impl Hub {
    ///
    /// Creates a new hub
    ///
    pub fn new() -> Hub {
        Hub { publisher: TreeChangeBus::new() }
    }

    ///
    /// Returns a consumer that will read from a particular address relative to this hub
    ///
    pub fn read_from<T: ToTreeAddress>(&mut self, address: &T) -> ConsumerRef {
        unimplemented!();
    }

    ///
    /// Returns a publisher that will write to a particular address relative to this hub
    ///
    pub fn publish_to<T: ToTreeAddress>(&mut self, address: &T) -> PublisherRef {
        unimplemented!();
    }
}
