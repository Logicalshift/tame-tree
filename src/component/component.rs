use std::rc::*;
use std::ops::*;

pub type PublisherRef = Rc<Publisher>;
pub type ConsumerRef = Rc<Consumer>;

///
/// A publisher reports changes to a tree
///
pub trait Publisher {

}

///
/// A consumer subscribes to published changes to a tree
///
pub trait Consumer {

}

///
/// PublisherRefs can be treated directly as publishers for convenience
///
impl Publisher for PublisherRef {

}

///
/// ConsumerRefs can be treated directly as consumers for convenience
///
impl Consumer for ConsumerRef {

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
