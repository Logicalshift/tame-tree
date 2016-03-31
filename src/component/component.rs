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
/// Types that implement this trait can be converted into components.
///
/// A component is an object that consumes tree changes from a consumer and publishes its output to a publisher.
///
pub trait ConvertToComponent {
    ///
    /// Converts this object into a component with a consumer and publisher. The object is consumed by this call.
    ///
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef;
}

///
/// Type that lets us avoid exposing boxing operations to the user
///
/// Let's say we want to implement a component converter for a function type `Fn(Foo)`. The type `Fn<Foo>` is 
/// not sized, so we could write this:
///
///   `impl<F> ConvertToComponent for F where F: Fn(Foo) { ... }`
///
/// but this will only work once, because rust's typechecker won't be able to distinguish it from Fn(Bar).
///
/// Alternatively, we could implement it like this:
///
///   `impl ConvertToComponent for Box(Fn(Foo)) { ... }` 
///
/// and make the user call `Box::new()` everywhere. That's kind of inconvenient for the user and seems
/// unnecessary as the converter takes possession of itself in order to build the component. This trait
/// lets us write this instead:
///
///   `impl BoxedConvertToComponent for Box(Fn(Foo)) { ... }`
///
/// and have it be possible to use Fn(Foo) directly as a component converter.
///
pub trait BoxedConvertToComponent {
    fn into_component_boxed(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef;
}

impl<F> ConvertToComponent for F where F: 'static, Box<F> : BoxedConvertToComponent {
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        Box::new(self).into_component_boxed(consumer, publisher)
    }
}

impl<F> ConvertToComponent for F where F:'static, Rc<F> : BoxedConvertToComponent {
    fn into_component(self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef {
        Rc::new(self).into_component_boxed(consumer, publisher)
    }
}
