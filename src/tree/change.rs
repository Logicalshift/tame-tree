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

use super::address::*;
use super::extent::*;
use super::treenode::*;
use super::basictree::*;
use super::values::*;

///
/// Represents the replacement action to perform on a particular tree node
///
#[derive(Clone)]
pub enum TreeReplacement {
    /// Removes this node
    Remove,

    /// Replaces the node with a new node
    NewNode(TreeRef),

    /// Changes the value of the node but leaves its children intact
    NewValue(String, TreeValue)
}

///
/// Trait implemented by things that can be converted to a tree replacement
///
pub trait ToTreeReplacement {
    ///
    /// Creates a representation of this object as a tree replacement
    ///
    fn to_tree_replacement(&self) -> TreeReplacement;
}

impl<T: ToTreeNode> ToTreeReplacement for T {
    #[inline]
    fn to_tree_replacement(&self) -> TreeReplacement {
        TreeReplacement::NewNode(self.to_tree_node())
    }
}

impl ToTreeReplacement for () {
    #[inline]
    fn to_tree_replacement(&self) -> TreeReplacement {
        TreeReplacement::Remove
    }
}

impl<T: ToTreeNode> ToTreeReplacement for Option<T> {
    #[inline]
    fn to_tree_replacement(&self) -> TreeReplacement {
        match *self {
            Some(ref tree_node) => TreeReplacement::NewNode(tree_node.to_tree_node()),
            None                => TreeReplacement::Remove
        }
    }
}

impl ToTreeReplacement for TreeReplacement {
    #[inline]
    fn to_tree_replacement(&self) -> TreeReplacement {
        self.clone()
    }
}

///
/// A change represents an alteration to the tree
///
pub struct TreeChange {
    /// The address of the node that should be changed (can be the address of a non-existent child to add a
    /// new child, or the address of the item after the last sibling to add a new sibling)
    address: TreeAddress,

    /// The tree that should replace the changed reference.
    ///
    /// The node at the specified address will be removed and this node will be added in its place. If this node is
    /// none, then the node at the address will be removed. If the node has 
    replacement: TreeReplacement
}

impl TreeChange {
    ///
    /// Creates a new tree change
    ///
    #[inline]
    pub fn new<TAddress: ToTreeAddress, TReplacement: ToTreeReplacement>(root: &TAddress, replacement: &TReplacement) -> TreeChange {
        TreeChange { address: root.to_tree_address(), replacement: replacement.to_tree_replacement() }
    }

    ///
    /// Finds the final sibling of an item and replaces it with a new sibling
    ///
    fn replace_sibling(node: &Option<TreeRef>, new_sibling: &Option<TreeRef>) -> Option<TreeRef> {
        if let Some(ref new_sibling_ref) = *new_sibling {
            // Push the existing siblings onto a stack
            let mut siblings    = vec![];
            let mut current     = match *node { Some(ref tree) => Some(tree.clone()), None => None };

            loop {
                if let Some(next_sibling) = current {
                    siblings.push(next_sibling.clone());

                    // Move on
                    current = next_sibling.get_sibling_ref();
                } else {
                    break;
                }
            }

            // Pop to generate the final result
            current = Some(new_sibling_ref.clone());

            while let Some(sibling) = siblings.pop() {
                current = Some(sibling.with_sibling_node(current.as_ref()));
            }

            current
        } else {
            node.clone()
        }
    }

    ///
    /// Returns how a replacement is applied to a particular tree node (or nothing) 
    ///
    fn perform_replacement(original: Option<&TreeRef>, replacement: &TreeReplacement) -> Option<TreeRef> {
        let original_sibling = original.and_then(|x| x.get_sibling_ref());
        let original_child   = original.and_then(|x| x.get_child_ref());

        match *replacement {
            TreeReplacement::Remove                         => original_sibling,
            TreeReplacement::NewNode(ref new_node)          => Self::replace_sibling(&Some(new_node.clone()), &original_sibling),
            TreeReplacement::NewValue(ref tag, ref value)   => Some(Rc::new(BasicTree::new(&*tag, value, original_child, original_sibling)))
        }
    }

    ///
    /// Performs the apply operation
    ///
    fn perform_apply(original: Option<&TreeRef>, address: &TreeAddress, replacement: &TreeReplacement) -> Option<TreeRef> {
        match *address {
            TreeAddress::Here => {
                // Just replace this node
                Self::perform_replacement(original, replacement)
            },

            TreeAddress::ChildAtIndex(child_index, ref child_address) => {
                // Copy the siblings into a stack
                let mut siblings    = vec![];
                let mut current     = original.and_then(|x| x.get_child_ref());

                for _ in 0..child_index {
                    siblings.push(current.clone().unwrap_or_else(|| Rc::new(BasicTree::new("", (), None, None))));

                    current = current.and_then(|x| x.get_sibling_ref());
                }

                // Replace the child at this index
                let mut new_child       = Self::perform_apply(current.as_ref(), &*child_address, replacement);

                // Pop siblings to generate the new child item
                current = new_child;
                while let Some(sibling) = siblings.pop() {
                    current = Some(sibling.with_sibling_node(current.as_ref()));
                }

                // Result is the original node with the new child node
                original.and_then(|x| Some(x.with_child_node(current.as_ref())))
            },

            TreeAddress::ChildWithTag(ref child_tag, ref child_address) => {
                unimplemented!();
            }
        }
    }
    
    ///
    /// Returns the result of applying this tree change to an existing tree
    ///
    #[inline]
    pub fn apply(&self, original_tree: &TreeRef) -> TreeRef {
        if let Some(result) = Self::perform_apply(Some(original_tree), &self.address, &self.replacement) {
            result
        } else {
            // If the change is 'delete the root node' then the result will be 'none' - we return an empty tree for that case
            "".to_tree_node()
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
        // TODO: if the change type is 'NewValue' then the change only applies if the address is exact
        Self::address_applies(&self.address, address)
    }

    ///
    /// Returns whether or not this change affects the child of a paticular address
    ///
    /// Corresponds to testing for an extent of `TreeExtent::Children`
    ///
    pub fn applies_to_child_of(&self, address: &TreeAddress) -> Option<bool> {
        self.address.parent().is_parent_of(address)
    }

    ///
    /// Returns whether or not this change affects only this address
    ///
    /// Corresponds to testing for an extent of `TreeExtent::ThisNode`
    ///
    pub fn applies_to_only(&self, address: &TreeAddress) -> Option<bool> {
        if let TreeReplacement::NewValue(_, _) = self.replacement {
            Some(self.address == *address)
        } else {
            self.address.is_parent_of(address)
        }
    }

    ///
    /// Returns with or not this change affects a node covered by a given extent relative to an address
    ///
    pub fn applies_to(&self, address: &TreeAddress, extent: &TreeExtent) -> Option<bool> {
        match *extent {
            TreeExtent::ThisNode    => self.applies_to_only(address),
            TreeExtent::Children    => self.applies_to_child_of(address),
            TreeExtent::SubTree     => self.applies_to_subtree(address)
        }
    }

    ///
    /// Generates a `NewNode` change using an address relative to an existing tree
    ///
    fn relative_to_tree(tree: &TreeRef, address: TreeAddress) -> Option<TreeChange> {
        let new_tree_maybe = tree.get_child_ref_at(address);

        if let Some(new_tree) = new_tree_maybe {
            Some(TreeChange::new(&TreeAddress::Here, &TreeReplacement::NewNode(new_tree)))
        } else {
            None
        }
    }

    ///
    ///
    fn adjust_root_address_for_partial_tree(&self, tree_address: &TreeAddress) -> Option<TreeAddress> {
        match *tree_address {
            TreeAddress::ChildAtIndex(index, ref remaining) => {
                match *self.address.last_part() {
                    TreeAddress::ChildAtIndex(our_index, _) => {
                        if (our_index <= index) {
                            Some(TreeAddress::ChildAtIndex(index - our_index, remaining.clone()))
                        } else {
                            None
                        }
                    },

                    _ => None
                }
            },

            _ => Some(tree_address.clone())
        }
    }

    ///
    /// Creates a new tree change that's relative to a subtree of the tree this change is for
    ///
    /// Ie, this reduces the scope of the change. If this change is for `.1.2.`, then asking for
    /// `relative_to(&1.to_tree_address())` will return a change for `.2.`.
    ///
    pub fn relative_to(&self, address: &TreeAddress) -> Option<TreeChange> {
        if address.is_parent_of(&self.address).unwrap_or(false) {
            // The changes are further down the tree: we can jsut change the root address
            let new_address_opt = self.address.relative_to(address);

            if let Some(new_address) = new_address_opt {
                Some(TreeChange::new(&new_address, &self.replacement))
            } else {
                None
            }
        } else {
            // The changes are within the change tree: we need to generate a new tree
            if let TreeReplacement::NewNode(ref tree) = self.replacement {
                match self.address {
                    TreeAddress::Here => {
                        // This change is already a straight up tree replacement
                        Self::relative_to_tree(tree, address.clone())
                    },

                    _ => {
                        // If the change affects sibligs or replaces a node, we need to navigate a tree containing the change
                        let parent_of_change: TreeRef   = Rc::new(BasicTree::new("", (), Some(tree.clone()), None));
                        let relative_to_tree_maybe      = address.relative_to(&self.address.parent()).and_then(|x| self.adjust_root_address_for_partial_tree(&x));

                        if let Some(relative_to_tree) = relative_to_tree_maybe {
                            Self::relative_to_tree(&parent_of_change, relative_to_tree)
                        } else {
                            None
                        }
                    }
                }
            } else {
                // Other change types don't create a tree so there is no result
                None
            }
        }
    }
}

#[cfg(test)]
mod change_tests {
    use super::super::super::tree::*;

    #[test]
    fn can_apply_simple_change_tagged() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&("two"), &("replaced", 4));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at("one").unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at("replaced").unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at("replaced").unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at("two").is_none());
        assert!(!changed_tree.get_child_ref_at("three").is_none());
    }

    #[test]
    fn can_apply_simple_change_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&1, &("replaced", 4));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at(3).is_none());
    }

    #[test]
    fn can_apply_remove_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&1, &TreeReplacement::Remove);
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at(2).is_none());
    }

    #[test]
    fn can_apply_new_value_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&1, &TreeReplacement::NewValue("replaced".to_string(), 4.to_tree_value()));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_sibling_ref().is_none());
        assert!(changed_tree.get_child_ref_at(3).is_none());
    }

    #[test]
    fn can_add_child_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&(1, 0), &("new_child", 4));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at((1,0).to_tree_address()).unwrap().get_value().to_int(0) == 4);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 2);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at(3).is_none());
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_sibling_ref().is_none());
    }

    #[test]
    fn can_insert_many_siblings_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&1, &("new_child", 4).to_tree_node().with_sibling_node(Some(&("new_child_2", 5).to_tree_node())));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(!changed_tree.get_child_ref_at(0).is_none());
        assert!(!changed_tree.get_child_ref_at(1).is_none());
        assert!(!changed_tree.get_child_ref_at(2).is_none());
        assert!(!changed_tree.get_child_ref_at(3).is_none());
        assert!(changed_tree.get_child_ref_at(4).is_none());

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_value().to_int(0) == 5);
        assert!(changed_tree.get_child_ref_at(3).unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at(3).unwrap().get_sibling_ref().is_none());
    }

    #[test]
    fn can_add_sibling_indexed() {
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&3, &("new_child", 4));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at(0).unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(1).unwrap().get_value().to_int(0) == 2);
        assert!(changed_tree.get_child_ref_at(2).unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at(3).unwrap().get_value().to_int(0) == 4);
    }

    #[test]
    fn can_add_child_tagged() {
        // Can address a non-existent node to add a new child or sibling
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&("one", "non_existent_node"), &("new_child", 4));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at("one").unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at(("one", "new_child").to_tree_address()).unwrap().get_value().to_int(0) == 4);
        assert!(changed_tree.get_child_ref_at("three").unwrap().get_value().to_int(0) == 3);
    }

    #[test]
    fn can_add_sibling_tagged() {
        // Can address a non-existent node to add a new child or sibling
        // The indexed structure after this operation is currently ill-defined
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_two      = TreeChange::new(&"non_existent_node", &("new_child", 4));
        let changed_tree    = change_two.apply(&initial_tree);

        assert!(changed_tree.get_child_ref_at("one").unwrap().get_value().to_int(0) == 1);
        assert!(changed_tree.get_child_ref_at("two").unwrap().get_value().to_int(0) == 2);
        assert!(changed_tree.get_child_ref_at("three").unwrap().get_value().to_int(0) == 3);
        assert!(changed_tree.get_child_ref_at("new_child").unwrap().get_value().to_int(0) == 4);
    }

    #[test]
    fn can_replace_entire_tree() {
        // The change is relative to an imaginary root, so replacing the child of . should replace the entire tree
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_all      = TreeChange::new(&TreeAddress::Here, &Some(("new_child", 4)));
        let changed_tree    = change_all.apply(&initial_tree);

        assert!(changed_tree.get_child_ref().is_none());
        assert!(changed_tree.get_sibling_ref().is_none());
        assert!(changed_tree.get_value().to_int(0) == 4);
    }

    #[test]
    fn can_replace_entire_tree_when_empty() {
        // The change is relative to an imaginary root, so replacing the child of . should replace the entire tree
        let initial_tree    = "empty".to_tree_node();
        let change_all      = TreeChange::new(&(), &Some(("new_child", 4)));
        let changed_tree    = change_all.apply(&initial_tree);

        assert!(changed_tree.get_child_ref().is_none());
        assert!(changed_tree.get_sibling_ref().is_none());
        assert!(changed_tree.get_value().to_int(0) == 4);
    }

    #[test]
    fn true_root_applies_to_subtree_everything() {
        // 'Delete whole tree'
        let change = TreeChange::new(&(), &());

        // Note that the tree change applies to (0, 1) but it's relative to an imaginary root
        assert!(change.applies_to_subtree(&().to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&(1).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_applies_to_subtree_child_tree() {
        // Remove .1.2.
        let change = TreeChange::new(&(1, 2), &());

        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&(1, (2, 3)).to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_applies_to_subtree_everything_up_to_root() {
        // Remove first child of .1.2.
        let change = TreeChange::new(&(1, (2, 0)), &());

        assert!(change.applies_to_subtree(&(1, 2).to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&1.to_tree_address()).unwrap());
        assert!(change.applies_to_subtree(&().to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_does_not_apply_to_sibling() {
        // Delete .1.2
        let change = TreeChange::new(&(1, 2), &());

        assert!(!change.applies_to_subtree(&(1, 1).to_tree_address()).unwrap());
        assert!(!change.applies_to_subtree(&2.to_tree_address()).unwrap());
    }

    #[test]
    fn child_change_does_not_apply_to_other_tree() {
        let change = TreeChange::new(&(1, 0).to_tree_address(), &());

        assert!(!change.applies_to_subtree(&(2, 2).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_child_only_true_for_changes_affecting_nodes_children() {
        let change = TreeChange::new(&(1, (2, 0)), &());

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_child_of(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_child_of(&(1).to_tree_address()).unwrap());

        // This will apply to the a child of the .1.2 node
        assert!(change.applies_to_child_of(&(1, 2).to_tree_address()).unwrap());

        // We've replaced .1.2 so the address .1.2.3 will be affected (as will .1.2.3.4, etc)
        assert!(change.applies_to_child_of(&(1, (2, 3)).to_tree_address()).unwrap());
        assert!(change.applies_to_child_of(&(1, (2, (3, 4))).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_only_true_for_exact_children() {
        let change = TreeChange::new(&(1, 2), &());

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_only(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_only(&(1).to_tree_address()).unwrap());

        // This will apply to the 1.2 node
        assert!(change.applies_to_only(&(1, 2).to_tree_address()).unwrap());

        // This change could affect the .1.2.3 node
        assert!(change.applies_to_only(&(1, (2, 3)).to_tree_address()).unwrap());

        // The .1.2.3.4 node could also be affected (as all the children of .1.2 are assumed to be replaced)
        assert!(change.applies_to_only(&(1, (2, (3, 4))).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_only_true_for_exact_item_if_change_is_value_only() {
        let change = TreeChange::new(&(1, 2), &TreeReplacement::NewValue("new_value".to_string(), 1.to_tree_value()));

        // Doesn't apply to things 'above' the change (the direct children of .1 are unaffected by the change)
        assert!(!change.applies_to_only(&().to_tree_address()).unwrap());
        assert!(!change.applies_to_only(&(1).to_tree_address()).unwrap());

        // This will apply to the 1.2 node
        assert!(change.applies_to_only(&(1, 2).to_tree_address()).unwrap());

        // As this is replacing the value of a single node, it won't apply to subtrees
        assert!(!change.applies_to_only(&(1, (2, 3)).to_tree_address()).unwrap());
        assert!(!change.applies_to_only(&(1, (2, (3, 4))).to_tree_address()).unwrap());
    }

    #[test]
    fn applies_to_dispatches_to_correct_function() {
        let change = TreeChange::new(&(1, (2, 0)), &());

        assert!(change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(change.applies_to(&(1, (2, 0)).to_tree_address(), &TreeExtent::ThisNode).unwrap());

        assert!(!change.applies_to(&2.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(!change.applies_to(&1.to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(!change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::ThisNode).unwrap());
    }

    #[test]
    fn relative_to_here_does_not_affect_change() {
        // The change is relative to an imaginary root, so replacing the child of . should replace the entire tree
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_all      = TreeChange::new(&(), &("new_child", 4)).relative_to(&TreeAddress::Here);
        let changed_tree    = change_all.unwrap().apply(&initial_tree);

        assert!(changed_tree.get_child_ref().is_none());
        assert!(changed_tree.get_sibling_ref().is_none());
        assert!(changed_tree.get_value().to_int(0) == 4);
    }

    #[test]
    fn relative_to_works_when_change_is_subtree() {
        let original_change = TreeChange::new(&(3, (4, (1, (2, 3)))), &());
        let relative_change = original_change.relative_to(&(3, 4).to_tree_address()).unwrap();

        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(relative_change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(relative_change.applies_to(&(1, (2, 3)).to_tree_address(), &TreeExtent::ThisNode).unwrap());

        assert!(!relative_change.applies_to(&2.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(!relative_change.applies_to(&1.to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(!relative_change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::ThisNode).unwrap());
    }

    #[test]
    fn relative_to_works_when_change_is_sibling() {
        let original_change = TreeChange::new(&1, &("new_child", 4).to_tree_node().with_sibling_node(Some(&("new_child_2", 5).to_tree_node())));
        let relative_change = original_change.relative_to(&2.to_tree_address()).unwrap();

        assert!(relative_change.applies_to(&().to_tree_address(), &TreeExtent::SubTree).unwrap());

        let original_tree   = ("empty").to_tree_node();
        let altered_tree    = relative_change.apply(&original_tree);

        assert!(altered_tree.get_value().to_int(0) == 5);
    }

    #[test]
    fn relative_to_works_when_change_is_sibling_tagged() {
        let original_change = TreeChange::new(&"one", &("new_child", 4).to_tree_node().with_sibling_node(Some(&("new_child_2", 5).to_tree_node())));
        let relative_change = original_change.relative_to(&"new_child_2".to_tree_address()).unwrap();

        assert!(relative_change.applies_to(&().to_tree_address(), &TreeExtent::SubTree).unwrap());

        let original_tree   = ("empty").to_tree_node();
        let altered_tree    = relative_change.apply(&original_tree);

        assert!(altered_tree.get_value().to_int(0) == 5);
    }

    #[test]
    fn relative_to_works_on_root_tree() {
        let original_change = TreeChange::new(&1, &());
        let relative_change = original_change.relative_to(&1.to_tree_address()).unwrap();

        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(relative_change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(relative_change.applies_to(&(1, (2, 3)).to_tree_address(), &TreeExtent::ThisNode).unwrap());

        assert!(relative_change.applies_to(&2.to_tree_address(), &TreeExtent::SubTree).unwrap());
        assert!(relative_change.applies_to(&1.to_tree_address(), &TreeExtent::Children).unwrap());
        assert!(relative_change.applies_to(&(1, 2).to_tree_address(), &TreeExtent::ThisNode).unwrap());
    }

    #[test]
    fn relative_to_works_when_change_is_larger_tree() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(1, 0), &tree!("one", tree!("two", tree!("three", "four"), "five")));

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
    fn relative_to_works_when_change_affects_entire_tree() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(), &tree!("root", ".0", tree!(".1", tree!("one", tree!("two", tree!("three", "four"), "five")))));

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
    fn relative_to_works_when_change_is_larger_tree_and_tagged() {
        // Change the child of ."root" to be the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        // Original tree is .root.old...
        // New tree is .root.one...
        let original_change = TreeChange::new(&("root", "old"), &tree!("one", tree!("two", tree!("three", "four"), "five")));

        // .root.one.two should represent the 'two' change
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
}
