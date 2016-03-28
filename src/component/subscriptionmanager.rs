use std::rc::*;
use super::super::util::clonecell::*;
use super::super::tree::*;

use super::component::*;

struct Subscription<TData: Clone> {
    callback: ConsumerCallback,
    data: TData
}

type SubscriptionRef<TData: Clone> = Rc<Subscription<TData>>;

///
/// The subscription manager is an interior mutable type that can store subscriptions created from consumers.
/// The principle use case is to make it so that publishers and consumers can share a list of subscriptions.
///
pub struct SubscriptionManager<TData: Clone> {
    subscriptions: CloneCell<Vec<SubscriptionRef<TData>>>
}

impl<TData: Clone> SubscriptionManager<TData> {
    ///
    /// Modifies this subscription manager to add the specified subscription
    ///
    pub fn add_subscription(&self, callback_data: TData, callback: ConsumerCallback) {
        // Turn the callback into a reference
        let new_callback = Rc::new(Subscription::<TData> { callback: callback, data: callback_data });

        // Retrieve and update the subscriptions
        let mut subscriptions = self.subscriptions.get();
        subscriptions.push(new_callback);
        self.subscriptions.set(subscriptions);
    }

    ///
    /// Calls the subscriptions matching a particular filter
    ///
    pub fn call_subscriptions(&self, call_filter: &Fn(&TData) -> bool, change: &TreeChange) {
        // Retrieve the active subscriptions
        let subscriptions = self.subscriptions.get();

        // Call any subscription matching the filter
        for possible_subscription in subscriptions {
            if call_filter(&possible_subscription.data) {
                let callback = &possible_subscription.callback;
                callback(change);
            }
        }
    }
}