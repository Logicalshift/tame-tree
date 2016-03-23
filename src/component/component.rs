use std::rc::*;
use std::ops::*;

use super::super::tree::*;

pub type PublisherRef = Box<Publisher>;
pub type ConsumerRef = Box<Consumer>;

///
/// A publisher reports changes to a tree
///
pub trait Publisher {

}

///
/// A consumer subscribes to published changes to a tree
///
pub trait Consumer {
    ///
    /// Calls a function whenever a particular section of the tree has changed
    ///
    fn subscribe(&mut self, address: TreeAddress, extent: TreeExtent, callback: Fn(TreeChange) -> ());
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
    fn create(&self, consumer: ConsumerRef, publisher: PublisherRef) -> ComponentRef;
}
