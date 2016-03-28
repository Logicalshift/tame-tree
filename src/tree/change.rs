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
                siblings.push(new_child);

                current = current.and_then(|x| x.get_sibling_ref());

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
                let child_tree  = original_tree.get_child_at(&**child_tag);
                let new_child   = TreeChange::perform_apply(&child_tree, &*child_address, change_type, replacement_tree);

                Rc::new(BasicTree::from_with_child(original_tree, new_child))
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
        let change_two      = TreeChange::new(&(0, 0), TreeChangeType::Sibling, Some(&("replaced", 4)));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at(2).is_none());
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
}
