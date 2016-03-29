use std::rc::*;

use super::address::*;
use super::extent::*;
use super::treenode::*;
use super::basictree::*;

///
/// Represents which of the root's references have changed
///
#[derive(Clone, Copy)]
pub enum TreeChangeType {
    /// The node's child reference has been replaced
    Child,

    /// The node's sibling reference has been replaced
    Sibling
}

///
/// A change represents an alteration to the tree
///
pub struct TreeChange {
    /// The address of the node matching the root of the change
    ///
    /// The address is relative to an imaginary node that is the parent of the real root node. This makes it possible to 
    /// replace the entire tree by setting this to `TreeAddress::Here` and the change_type to `TreeChangeType::Child`.
    /// The real root node can be addressed at `TreeAddress::ChildAtIndex(0)`
    root: TreeAddress,

    /// Which of the root's references have changed
    change_type: TreeChangeType,

    /// The tree that should replace the changed reference. The last node in this tree (depth-first) will be given the same sibling as the last node in the replacement range
    replacement_tree: Option<TreeRef>
}

impl TreeChange {
    ///
    /// Creates a new tree change
    ///
    pub fn new<TAddress: ToTreeAddress, TNode: ToTreeNode>(root: &TAddress, change_type: TreeChangeType, replacement_tree: Option<&TNode>) -> TreeChange {
        TreeChange { root: root.to_tree_address(), change_type: change_type, replacement_tree: replacement_tree.map(|x| x.to_tree_node()) }
    }

    ///
    /// Performs a replacement on a basic tree node
    ///
    #[inline]
    fn perform_replacement(node: &BasicTree, change_type: &TreeChangeType, replacement_tree: &Option<TreeRef>) {
        match *change_type {
            TreeChangeType::Child => {
                match *replacement_tree {
                    Some(ref new_child)     => node.set_child_ref(new_child.clone()),
                    None                    => node.clear_child()
                }
            },

            TreeChangeType::Sibling => {
                match *replacement_tree {
                    Some(ref new_sibling)   => node.set_sibling_ref(new_sibling.clone()),
                    None                    => node.clear_child()
                }
            }
        }
    }

    ///
    /// Applies a change to a tree, generating a new tree
    ///
    fn perform_apply(original_tree: &TreeRef, address: &TreeAddress, change_type: &TreeChangeType, replacement_tree: &Option<TreeRef>) -> TreeRef {
        match *address {
            TreeAddress::Here => {
                let new_node = BasicTree::from(original_tree);
                TreeChange::perform_replacement(&new_node, change_type, replacement_tree);

                Rc::new(new_node)
            },

            TreeAddress::ChildAtIndex(child_index, ref child_address) => {
                // Copy the siblings into a stack
                let mut siblings    = vec![];
                let mut current     = original_tree.get_child_ref();

                for _ in 0..child_index {
                    siblings.push(current.clone().unwrap_or_else(|| Rc::new(BasicTree::new("", ()))));

                    current = current.and_then(|x| x.get_sibling_ref());
                }

                // Replace the child matching this item
                let child_tree  = current.clone().unwrap_or_else(|| Rc::new(BasicTree::new("", ())));
                let new_child   = TreeChange::perform_apply(&child_tree, &*child_address, change_type, replacement_tree);

                current = Some(new_child);

                // Pop siblings to generate the new child item
                while let Some(sibling) = siblings.pop() {
                    match current {
                        Some(next_sibling)  => current = Some(Rc::new(BasicTree::from_with_sibling(sibling, next_sibling))),
                        None                => current = Some(sibling),
                    }
                }

                Rc::new(BasicTree::from_with_child(original_tree, current.unwrap()))
            },

            TreeAddress::ChildWithTag(ref child_tag, ref child_address) => {
                // Copy the siblings into a stack
                let mut siblings    = vec![];
                let mut current     = original_tree.get_child_ref();

                while current.is_some() {
                    if let Some(ref child_ref) = current {
                        // Always true, but it isn't clear how to make while let work here when we need to update the pointer (because it's always borrowed in the loop block)
                        if child_ref.get_tag() == child_tag {
                            break;
                        }

                        siblings.push(child_ref.clone());
                    }

                    current = current.and_then(|x| x.get_sibling_ref());
                }

                // Replace the child matching this item
                let child_tree  = current.clone().unwrap_or_else(|| Rc::new(BasicTree::new("", ())));
                let new_child   = TreeChange::perform_apply(&child_tree, &*child_address, change_type, replacement_tree);

                current = Some(new_child);

                // Pop siblings to generate the new child item
                while let Some(sibling) = siblings.pop() {
                    match current {
                        Some(next_sibling)  => current = Some(Rc::new(BasicTree::from_with_sibling(sibling, next_sibling))),
                        None                => current = Some(sibling),
                    }
                }

                Rc::new(BasicTree::from_with_child(original_tree, current.unwrap()))
            }
        }
    }

    ///
    /// Creates a new tree from an old tree with this change added to it
    ///
    #[inline]
    pub fn apply(&self, original_tree: &TreeRef) -> TreeRef {
        // For simplicity, we actualise the 'imaginary' root
        let imaginary_root      = tree!("", original_tree).to_tree_node();
        let new_imaginary_root  = TreeChange::perform_apply(&imaginary_root, &self.root, &self.change_type, &self.replacement_tree);

        // Result is the child of the imaginary node (or an empty node if the entire tree is deleted)
        new_imaginary_root.get_child_ref().unwrap_or_else(|| "".to_tree_node())
    }

    ///
    /// Converts the root address to one that's relative to the tree being changed
    ///
    /// The root address starts at an imaginary 'true' root (this makes it possible to specify a change that replaces the entire tree)
    ///
    #[inline]
    fn address_relative_to_tree_root(&self) -> &TreeAddress {
        static HERE: TreeAddress = TreeAddress::Here;

        match self.root {
            TreeAddress::Here                           => &HERE,
            TreeAddress::ChildAtIndex(0, ref address)   => &**address,
            _                                           => &HERE
        }
    }

    ///
    /// Determines if a change to a particular address will also affect the value of a different address
    ///
    #[inline]
    fn address_applies(changing_address: &TreeAddress, testing_address: &TreeAddress) -> Option<bool> {
        let is_parent_of_changing = changing_address.is_parent_of(testing_address);

        match is_parent_of_changing {
            None | Some(false)  => testing_address.is_parent_of(changing_address),
            _                   => is_parent_of_changing
        }
    }

    ///
    /// Returns whether or not this change covers the specified address (or false if this cannot be determined)
    ///
    /// Corresponds to testing for an extent of `TreeExtent::SubTree`
    ///
    pub fn applies_to_subtree(&self, address: &TreeAddress) -> Option<bool> {
        let relative_root = self.address_relative_to_tree_root();

        match self.change_type {
            // If the child has changed, then anything that's a child of the root address is changed
            TreeChangeType::Child => TreeChange::address_applies(relative_root, address),

            // If the sibling has changed, then it's the parent address that's changed
            TreeChangeType::Sibling => TreeChange::address_applies(&relative_root.parent(), address)
        }
    }

    ///
    /// Returns whether or not this change affects the child of a paticular address
    ///
    /// Corresponds to testing for an extent of `TreeExtent::Children`
    ///
    pub fn applies_to_child_of(&self, address: &TreeAddress) -> Option<bool> {
        let relative_root = self.address_relative_to_tree_root();

        match self.change_type {
            // If the child has changed, then anything that's a child of the root address is changed
            TreeChangeType::Child => relative_root.is_parent_of(address),

            // If the sibling has changed, then it's the parent address that's changed
            TreeChangeType::Sibling => relative_root.parent().is_parent_of(address)
        }
    }

    ///
    /// Returns whether or not this change affects only this address
    ///
    /// Corresponds to testing for an extent of `TreeExtent::ThisNode`
    ///
    pub fn applies_to_only(&self, address: &TreeAddress) -> Option<bool> {
        let relative_root = self.address_relative_to_tree_root();

        match self.change_type {
            TreeChangeType::Child => relative_root.is_parent_of(&address.parent()),
            TreeChangeType::Sibling => relative_root.parent().is_parent_of(&address.parent())
        }
    }

    ///
    /// Returns with or not this change affects a node covered by a given extent relative to an address
    ///
    #[inline]
    pub fn applies_to(&self, address: &TreeAddress, extent: &TreeExtent) -> Option<bool> {
        match *extent {
            TreeExtent::ThisNode    => self.applies_to_only(address),
            TreeExtent::Children    => self.applies_to_child_of(address),
            TreeExtent::SubTree     => self.applies_to_subtree(address)
        }
    }

    ///
    /// Creates a new tree change that's relative to a subtree of the tree this change is for
    ///
    pub fn relative_to(&self, address: &TreeAddress) -> Option<TreeChange> {
        // Get the new root relative to the main tree
        let new_root_opt = self.address_relative_to_tree_root().relative_to(address);

        if let Some(new_root) = new_root_opt {
            // Prepend the '0' needed to deal with the 'imaginary' root
            let new_root_relative = (0, new_root).to_tree_address();

            Some(TreeChange::new(&new_root_relative, self.change_type, self.replacement_tree.as_ref()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod change_tests {
    use super::super::super::tree::*;

    #[test]
    fn can_apply_simple_change_tagged() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&("test", "one"), TreeChangeType::Sibling, Some(&("replaced", 4)));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at("one").unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at("replaced").unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at("replaced").unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at("two").is_none());
        assert!(changed_tree.get_child_ref_at("three").is_none());
    }

    #[test]
    fn can_apply_simple_change_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&(0, 1), TreeChangeType::Sibling, Some(&("replaced", 4)));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 2);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at(3).is_none());
    }

    #[test]
    fn can_apply_child_change_tagged() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&("test", "two").to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(!changed_tree.get_child_ref_at(("two", "new_child").to_tree_address()).is_none());

        assert!(changed_tree.get_child_ref_at(("two", "new_child").to_tree_address()).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at(("two", "new_child").to_tree_address()).unwrap().get_sibling_ref().is_none());

        assert!(!changed_tree.get_child_ref_at("one").is_none());
        assert!(!changed_tree.get_child_ref_at("two").is_none());
        assert!(!changed_tree.get_child_ref_at("three").is_none());
        assert!(changed_tree.get_child_ref_at("two").unwrap().get_value().to_int(0) == 2);
    }

    #[test]
    fn can_apply_child_change_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(!changed_tree.get_child_ref_at((1, 0).to_tree_address()).is_none());

        assert!(changed_tree.get_child_ref_at((1, 0).to_tree_address()).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at((1, 0).to_tree_address()).unwrap().get_sibling_ref().is_none());

        assert!(!changed_tree.get_child_ref_at(0).is_none());
        assert!(!changed_tree.get_child_ref_at(1).is_none());
        assert!(!changed_tree.get_child_ref_at(2).is_none());
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 2);
    }

    #[test]
    fn true_root_applies_to_subtree_everything() {
        // The child of address 0 represents the entire tree
        let change = TreeChange::new(&0.to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        // Note that the tree change applies to (0, 1) but it's relative to an imaginary root
        assert!(change.applies_to_subtree(&().to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&(1).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_applies_to_subtree_child_tree() {
        let change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        // Note that the tree change applies to (0, 1) but it's relative to an imaginary root
        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&(1, (2, 3)).to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_applies_to_subtree_everything_up_to_root() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&1.to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&().to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_does_not_apply_to_sibling() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        assert!(!change.applies_to_subtree(&(1, 1).to_tree_address()).unwrap());
        assert!(!change.applies_to_subtree(&2.to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_does_not_apply_to_other_tree() {
        let change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        assert!(!change.applies_to_subtree(&(2, 2).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_subtree_parent() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        assert!(change.applies_to_subtree(&1.to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_subtree_sibling() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        assert!(change.applies_to_subtree(&(1, 3).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_subtree_child() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        assert!(change.applies_to_subtree(&(1, (2, 3)).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_does_not_apply_to_parent_sibling() {
        let change = TreeChange::new(&(0, (1, (2, 3))).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        assert!(!change.applies_to_subtree(&2.to_tree_address()).unwrap());
        assert!(!change.applies_to_subtree(&(1, 3).to_tree_address()).unwrap());
        assert!(!change.applies_to_subtree(&(1, (3, 4)).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_subtree_everything_up_to_root() {
        let change = TreeChange::new(&(0, (1, (2, 3))).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&1.to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&().to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_child_only_true_for_changes_affecting_nodes_children() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_child_of(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_child_of(&(1).to_tree_address()).unwrap());

        // This will apply to the children of the .1.2 node
        assert!(change.applies_to_child_of(&(1, 2).to_tree_address()).unwrap());

        // We've replaced the child of .1.2 so the address .1.2.3 will be affected (as will .1.2.3.4, etc)
        assert!(change.applies_to_child_of(&(1, (2, 3)).to_tree_address()).unwrap());
        assert!(change.applies_to_child_of(&(1, (2, (3, 4))).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_child_only_true_for_changes_affecting_nodes_children_with_siblings() {
        let change = TreeChange::new(&(0, (1, (2, 3))).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_child_of(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_child_of(&(1).to_tree_address()).unwrap());

        // This will apply to the children of the .1.2 node
        assert!(change.applies_to_child_of(&(1, 2).to_tree_address()).unwrap());

        // We've replaced a sibling of .1.2.3 so the address .1.2.3 will be affected (as will .1.2.4, etc)
        // For simplicity we also specify that .1.2.2 and lower are affected by the change too
        assert!(change.applies_to_child_of(&(1, (2, 1)).to_tree_address()).unwrap());
        assert!(change.applies_to_child_of(&(1, (2, (5, 6))).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_only_true_for_exact_children() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_only(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_only(&(1).to_tree_address()).unwrap());

        // This will apply to the children of the .1.2 node (which won't be affected directly)
        assert!(!change.applies_to_only(&(1, 2).to_tree_address()).unwrap());

        // This change could affect the .1.2.3 node
        assert!(change.applies_to_only(&(1, (2, 3)).to_tree_address()).unwrap());

        // The .1.2.3.4 node could also be affected (as all the children of .1.2 are assumed to be replaced)
        assert!(change.applies_to_only(&(1, (2, (3, 4))).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_only_true_for_exact_children_with_siblings() {
        let change = TreeChange::new(&(0, (1, (2, 3))).to_tree_address(), TreeChangeType::Sibling, None::<&TreeRef>);

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_only(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_only(&(1).to_tree_address()).unwrap());

        // This will apply to the children of the .1.2 node
        assert!(!change.applies_to_only(&(1, 2).to_tree_address()).unwrap());

        // This change could affect the .1.2.3 node
        assert!(change.applies_to_only(&(1, (2, 3)).to_tree_address()).unwrap());

        // The .1.2.3.4 node could also be affected
        assert!(change.applies_to_only(&(1, (2, (3, 4))).to_tree_address()).unwrap());

        // The .1.2.1 node technically couldn't be affected, but we'll return it too
        assert!(change.applies_to_only(&(1, (2, 1)).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_dispatches_to_correct_function() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);

        assert!(change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(change.applies_to(&(1, (2, 3)).to_tree_address(), &TreeExtent::ThisNode).unwrap());

        assert!(!change.applies_to(&2.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(!change.applies_to(&1.to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(!change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::ThisNode).unwrap());
    }

    #[test]
    fn relative_to_works_when_change_is_subtree() {
        let original_change = TreeChange::new(&(0, (3, (4, (1, 2)))).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);
        let relative_change = original_change.relative_to(&(3, 4).to_tree_address()).unwrap();

        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(relative_change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(relative_change.applies_to(&(1, (2, 3)).to_tree_address(), &TreeExtent::ThisNode).unwrap());

        assert!(!relative_change.applies_to(&2.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(!relative_change.applies_to(&1.to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(!relative_change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::ThisNode).unwrap());
    }

    #[test]
    fn relative_to_works_when_change_is_larger_tree() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

        // .1.0.0 should represent the 'two' change
        let relative_change = original_change.relative_to(&(1, (0, 0)).to_tree_address()).unwrap();

        // 'three', the first child of the 'two' node
        assert!(relative_change.applies_to(&0.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // 'five', the second child
        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // Should be able to apply to the empty tree
        let empty_tree      = tree!("empty", "");
        let changed_tree    = relative_change.apply(&empty_tree);

        assert!(changed_tree.get_tag() == "two");
        assert!(changed_tree.get_child_at(0).get_tag() == "three");
    }

    #[test]
    fn relative_to_works_when_change_is_larger_tree_and_sibling() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Sibling, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

        // .1.0.0 should represent the 'two' change
        let relative_change = original_change.relative_to(&(2, 0).to_tree_address()).unwrap();

        // 'three', the first child of the 'two' node
        assert!(relative_change.applies_to(&0.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // 'five', the second child
        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // Should be able to apply to the empty tree
        let empty_tree      = tree!("empty", "");
        let changed_tree    = relative_change.apply(&empty_tree);

        assert!(changed_tree.get_tag() == "two");
        assert!(changed_tree.get_child_at(0).get_tag() == "three");
    }

    #[test]
    fn relative_to_works_when_change_is_larger_tree_and_tagged() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(0, "root").to_tree_address(), TreeChangeType::Child, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

        // .1.0.0 should represent the 'two' change
        let relative_change = original_change.relative_to(&("root", ("one", "two")).to_tree_address()).unwrap();

        // 'three', the first child of the 'two' node
        assert!(relative_change.applies_to(&0.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // 'five', the second child
        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // Should be able to apply to the empty tree
        let empty_tree      = tree!("empty", "");
        let changed_tree    = relative_change.apply(&empty_tree);

        assert!(changed_tree.get_tag() == "two");
        assert!(changed_tree.get_child_at(0).get_tag() == "three");
    }

    #[test]
    fn relative_to_works_when_change_is_larger_tree_and_tagged_sibling() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(0, "root").to_tree_address(), TreeChangeType::Sibling, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

        // .1.0.0 should represent the 'two' change
        let relative_change = original_change.relative_to(&("one", "two").to_tree_address()).unwrap();

        // 'three', the first child of the 'two' node
        assert!(relative_change.applies_to(&0.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // 'five', the second child
        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());

        // Should be able to apply to the empty tree
        let empty_tree      = tree!("empty", "");
        let changed_tree    = relative_change.apply(&empty_tree);

        assert!(changed_tree.get_tag() == "two");
        assert!(changed_tree.get_child_at(0).get_tag() == "three");
    }
}
