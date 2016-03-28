use std::rc::*;
use super::super::util::clonecell::*;
use super::super::tree::*;

use super::component::*;

struct Subscription<TData: Clone> {
    callback: ConsumerCallback,
    data: TData
}

type SubscriptionRef<TData> = Rc<Subscription<TData>>;

///
/// The subscription manager is an interior mutable type that can store subscriptions created from consumers.
/// The principle use case is to make it so that publishers and consumers can share a list of subscriptions.
///
pub struct SubscriptionManager<TData: Clone> {
    subscriptions: CloneCell<Vec<SubscriptionRef<TData>>>
}

impl<TData: Clone> SubscriptionManager<TData> {
    ///
    /// Creates a new subscription manager
    ///
    pub fn new() -> SubscriptionManager<TData> {
        SubscriptionManager { subscriptions: CloneCell::new(vec![]) }
    }

    ///
    /// Modifies this subscription manager to add the specified subscription
    ///
    pub fn add_subscription(&self, callback_data: TData, callback: ConsumerCallback) {
        // Turn the callback into a reference
        let new_callback = Rc::new(Subscription { callback: callback, data: callback_data });

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

#[cfg(test)]
mod subscriptionmanager_tests {
    use std::rc::*;
    use std::cell::*;

    use super::*;
    use super::super::super::tree::*;

    #[test]
    pub fn can_call_subscription() {
        // Create a subscription manager and a sample change (doesn't matter what the change is)
        let manager         = SubscriptionManager::<i32>::new();
        let a_change        = TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"".to_tree_node()));

        // Store the change count in a shared cell
        let change_count    = Rc::new(Cell::<i32>::new(0));
        let callback_count  = change_count.clone();

        // Must initially be 0
        assert!(change_count.get() == 0);

        // Create a subscription that updates the change count
        manager.add_subscription(0, Box::new(move |_change: &TreeChange| { 
            let count_value = callback_count.get();
            let new_value   = count_value + 1;
            callback_count.set(new_value);
        }));

        // Call the subscription a few times (result should update)
        manager.call_subscriptions(&|_data| { true }, &a_change);
        assert!(change_count.get() == 1);
        manager.call_subscriptions(&|_data| { true }, &a_change);
        assert!(change_count.get() == 2);
    }

    #[test]
    pub fn can_filter_all_subscriptions() {
        // Create a subscription manager and a sample change (doesn't matter what the change is)
        let manager         = SubscriptionManager::<i32>::new();
        let a_change        = TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"".to_tree_node()));

        // Store the change count in a shared cell
        let change_count    = Rc::new(Cell::<i32>::new(0));
        let callback_count  = change_count.clone();

        // Must initially be 0
        assert!(change_count.get() == 0);

        // Create a subscription that updates the change count
        manager.add_subscription(0, Box::new(move |_change: &TreeChange| { 
            let count_value = callback_count.get();
            let new_value   = count_value + 1;
            callback_count.set(new_value);
        }));

        // Call the subscription a few times (result should not update)
        manager.call_subscriptions(&|_data| { false }, &a_change);
        assert!(change_count.get() == 0);
        manager.call_subscriptions(&|_data| { false }, &a_change);
        assert!(change_count.get() == 0);
    }

    #[test]
    pub fn can_filter_some_subscriptions() {
        // Create a subscription manager and a sample change (doesn't matter what the change is)
        let manager         = SubscriptionManager::<i32>::new();
        let a_change        = TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&"".to_tree_node()));

        // Store the change count in a shared cell
        let change_count    = Rc::new(Cell::<i32>::new(0));
        let callback_count  = change_count.clone();
        let callback_count2 = change_count.clone();

        // Must initially be 0
        assert!(change_count.get() == 0);

        // Create a subscription that updates the change count
        manager.add_subscription(0, Box::new(move |_change: &TreeChange| { 
            let count_value = callback_count.get();
            let new_value   = count_value + 1;
            callback_count.set(new_value);
        }));
        manager.add_subscription(1, Box::new(move |_change: &TreeChange| { 
            let count_value = callback_count2.get();
            let new_value   = count_value + 1;
            callback_count2.set(new_value);
        }));

        // Call the subscription a few times (result should not update)
        manager.call_subscriptions(&|_data| { true }, &a_change);
        assert!(change_count.get() == 2);
        manager.call_subscriptions(&|_data| { false }, &a_change);
        assert!(change_count.get() == 2);
        manager.call_subscriptions(&|data| { *data == 1 }, &a_change);
        assert!(change_count.get() == 3);
    }
}
