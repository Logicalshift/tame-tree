use std::rc::*;

use super::address::*;
use super::treenode::*;
use super::basictree::*;

///
/// Represents which of the root's references have changed
///
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
    fn address_relative_to_tree_root<'a>(&'a self) -> &'a TreeAddress {
        static HERE: TreeAddress = TreeAddress::Here;

        match self.root {
            TreeAddress::Here                           => &HERE,
            TreeAddress::ChildAtIndex(0, ref address)   => &**address,
            _                                           => &HERE
        }
    }

    ///
    /// Returns whether or not this change covers the specified address (or false if this cannot be determined)
    ///
    pub fn applies_to(&self, address: &TreeAddress) -> Option<bool> {
        let relative_root = self.address_relative_to_tree_root();

        match self.change_type {
            // If the child has changed, then anything that's a child of the root address is changed
            TreeChangeType::Child => relative_root.is_parent_of(address).map(|is_parent| { is_parent && *relative_root != *address }),

            // If the sibling has changed, then it's the parent address that's changed
            TreeChangeType::Sibling => relative_root.parent().is_parent_of(address)
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
    fn true_root_applies_to_everything() {
        // The child of address 0 represents the entire tree
        let change = TreeChange::new(&0.to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));

        // Note that the tree change applies to (0, 1) but it's relative to an imaginary root
        assert!(change.applies_to(&().to_tree_address()).unwrap());
        assert!(change.applies_to(&(1).to_tree_address()).unwrap());
        assert!(change.applies_to(&(1, 2).to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_applies_to_child_tree() {
        let change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));

        // Note that the tree change applies to (0, 1) but it's relative to an imaginary root
        assert!(change.applies_to(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to(&(1, (2, 3)).to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_applies_to_everything_up_to_root() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));

        assert!(change.applies_to(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to(&1.to_tree_address()).unwrap());
        assert!(change.applies_to(&().to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_does_not_apply_to_sibling() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));

        assert!(!change.applies_to(&(1, 1).to_tree_address()).unwrap());
        assert!(!change.applies_to(&2.to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_does_not_apply_to_other_tree() {
        let change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, Some(&("new_child", 4)));

        assert!(!change.applies_to(&(2, 2).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_parent() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, Some(&("new_child", 4)));

        assert!(change.applies_to(&1.to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_sibling() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, Some(&("new_child", 4)));

        assert!(change.applies_to(&(1, 3).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_child() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, Some(&("new_child", 4)));

        assert!(change.applies_to(&(1, (2, 3)).to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_does_not_apply_to_parent_sibling() {
        let change = TreeChange::new(&(0, (1, 2)).to_tree_address(), TreeChangeType::Sibling, Some(&("new_child", 4)));

        assert!(!change.applies_to(&2.to_tree_address()).unwrap());
    }

    #[test]
    fn sibling_change_applies_to_everything_up_to_root() {
        let change = TreeChange::new(&(0, (1, (2, 3))).to_tree_address(), TreeChangeType::Sibling, Some(&("new_child", 4)));

        assert!(!change.applies_to(&(0, (1, 2)).to_tree_address()).unwrap());
        assert!(!change.applies_to(&(0, 1).to_tree_address()).unwrap());
        assert!(!change.applies_to(&1.to_tree_address()).unwrap());
        assert!(!change.applies_to(&().to_tree_address()).unwrap());
    }
}
