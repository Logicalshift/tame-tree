use std::rc::*;
use super::super::util::clonecell::*;

use super::component::*;

type CallbackRef = Rc<ConsumerCallback>;

///
/// The subscription manager is an interior mutable type that can store subscriptions created from consumers
///
pub struct SubscriptionManager {
    subscriptions: CloneCell<Vec<CallbackRef>>
}

impl SubscriptionManager {
    ///
    /// Modifies this subscription manager to add the specified subscription
    ///
    pub fn add_subscription(&self, callback: ConsumerCallback) {
        // Turn the callback into a reference
        let new_callback = Rc::new(callback);

        // Retrieve and update the subscriptions
        let mut subscriptions = self.subscriptions.get();
        subscriptions.push(new_callback);
        self.subscriptions.set(subscriptions);
    }
}