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

///
/// Represents which of the root's references have changed
///
#[derive(Clone, Copy, PartialEq)]
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
    /// Given an address that matches a parent of the address where this change will take place, returns a new change relative
    /// to the subtree represented by that address.
    ///
    #[inline]
    fn relative_to_parent(&self, parent_address: &TreeAddress) -> Option<TreeChange> {
        let relative_root = self.address_relative_to_tree_root();

        // Get the new root relative to the main tree
        if let Some(new_root) = relative_root.relative_to(parent_address) {
            // Prepend the '0' needed to deal with the 'imaginary' root
            let new_root_relative = (0, new_root).to_tree_address();

            Some(TreeChange::new(&new_root_relative, self.change_type, self.replacement_tree.as_ref()))
        } else {
            None
        }
    }

    ///
    /// Given an address that matches a child of the address where this change will take place, returns a new change relative
    /// to the subtree represented by that address.
    ///
    #[inline]
    fn relative_to_child(&self, child_address: &TreeAddress) -> Option<TreeChange> {
        let relative_root = self.address_relative_to_tree_root();

        // Get the address of the part of the changed tree that will apply to the new fix
        if let Some(root_relative_to_tree) = child_address.relative_to(&relative_root) {
            self.relative_to_replacement_tree(root_relative_to_tree)
        } else {
            // Change doesn't affect this tree
            None
        }
    }

    ///
    /// Given an address that matches a sibling of the address where this change will take place, returns a new change relative
    /// to the subtree represented by that address.
    ///
    fn relative_to_sibling(&self, sibling_address: &TreeAddress) -> Option<TreeChange> {
        // If we change the sibling of a node like .1.2, then the replacement tree will have the node .1.3
        // If want to look up, say, .1.4.2, we need to look up the .2 subtree in the first sibling of the replacement
        // tree node. To do this, we create a fake tree with its child set to the replacement tree, and look up 
        // .1.2 in it (calculating the .1 is the main thing we need to do: it's 4-2-1: that is, the index in the
        // 'real' tree, adjusted to match a tree where the replacement is the only child)

        // This change may occur before the address: we're changing a sibling, so we're replacing a part of a tree
        let relative_root = self.address_relative_to_tree_root();

        // Get the parent of the node whose sibling is changing
        let parent_address              = relative_root.parent();
        let maybe_relative_to_parent    = sibling_address.relative_to(&parent_address);

        if let Some(relative_to_parent) = maybe_relative_to_parent {
            // Possible that the address is within the tree
            match relative_to_parent {
                TreeAddress::ChildAtIndex(index, ref remaining_address) => {
                    // If we create a fake root node, then the child nodes will be offset by the index of the last part of relative_root
                    let last_part_of_root = relative_root.last_part();

                    if let &TreeAddress::ChildAtIndex(offset_index, _) = last_part_of_root {
                        // Generate an offset address
                        if index > offset_index {
                            // Relative address is after the modification
                            let modified_relative_to_parent = TreeAddress::ChildAtIndex(index - offset_index - 1, remaining_address.clone());
                            self.relative_to_replacement_tree(modified_relative_to_parent)
                        } else {
                            // Relative address is before the modification
                            None
                        }
                    } else {
                        // Last part is not an offset address, so we can't adjust the first part of the relative address
                        None
                    }
                },

                TreeAddress::ChildWithTag(_, _) => {
                    // With a tag lookup, we can just create a fake root node and find the tag if it exists in the replacement tree
                    self.relative_to_replacement_tree(relative_to_parent)
                },

                _ => None
            }
        } else {
            None
        }
    }

    ///
    /// Given an address relative to the tree node that is the parent of the replacement tree, returns a new change relative
    /// to that address.
    ///
    #[inline]
    fn relative_to_replacement_tree(&self, tree_address: TreeAddress) -> Option<TreeChange> {
        // root_relative_to_tree is the address relative to the node that is having its child replaced. 
        // The tree itself represents the first child. Make a fake node to represent the node that will be replaced
        let fake_root_node = Rc::new(BasicTree::new("", ()));
        if let Some(ref replacement_child) = self.replacement_tree {
            fake_root_node.set_child_ref(replacement_child.clone());
        }

        // Get the new tree relative to the fake tree
        if let Some(new_change_tree) = fake_root_node.get_child_ref_at(tree_address) {
            // Tree is being replaced by a new tree
            Some(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&new_change_tree)))
        } else {
            // Tree is being deleted
            Some(TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, None::<&TreeRef>))
        }
    }

    ///
    /// Creates a new tree change that's relative to a subtree of the tree this change is for
    ///
    pub fn relative_to(&self, address: &TreeAddress) -> Option<TreeChange> {
        let relative_root = self.address_relative_to_tree_root();

        if address.is_parent_of(relative_root).unwrap_or(false) {
            // This change is further down the tree from the address: we can simply change the root address
            self.relative_to_parent(address)
        } else if self.change_type == TreeChangeType::Child && relative_root.is_parent_of(address).unwrap_or(false) {
            // This change occurs before the address: we need to trim the tree to accomodate it
            self.relative_to_child(address)
        } else if self.change_type == TreeChangeType::Sibling {
            self.relative_to_sibling(address)
        } else {
            // This change does not affect this address
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
    fn can_replace_entire_tree() {
        // The change is relative to an imaginary root, so replacing the child of . should replace the entire tree
        let initial_tree    = tree!("test", ("one", 1), ("two", 2), ("three", 3));
        let change_all      = TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&("new_child", 4)));
        let changed_tree    = change_all.apply(&initial_tree);

        assert!(changed_tree.get_child_ref().is_none());
        assert!(changed_tree.get_sibling_ref().is_none());
        assert!(changed_tree.get_value().to_int(0) == 4);
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
    fn relative_to_works_on_root_tree() {
        let original_change = TreeChange::new(&(0, 1).to_tree_address(), TreeChangeType::Child, None::<&TreeRef>);
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
        let original_change = TreeChange::new(&(0, 1), TreeChangeType::Child, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

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
        let original_change = TreeChange::new(&TreeAddress::Here, TreeChangeType::Child, Some(&tree!("root", ".0", tree!(".1", tree!("one", tree!("two", tree!("three", "four"), "five"))))));

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
        let original_change = TreeChange::new(&(0, 1), TreeChangeType::Sibling, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

        // .1. should represent the top of the tree, so .2. will be the 'one' node
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
        let original_change = TreeChange::new(&(0, "root"), TreeChangeType::Child, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

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

    #[test]
    fn relative_to_works_when_change_is_larger_tree_and_tagged_sibling() {
        // Change the child of .1 to have the subtree one -> two -> three (ie, we get a tree .1.0.0.0)
        let original_change = TreeChange::new(&(0, "root"), TreeChangeType::Sibling, Some(&tree!("one", tree!("two", tree!("three", "four"), "five"))));

        // .one. will represent the sibling of .root. after the change
        // There's a gotcha with this: the change has no way to know if .root. occurs after another .one. (as tagged addresses don't 
        // have to be unique). If it's after another .one. then the relative change will be referring to the 'wrong' part of the tree.
        // TODO: we could fix this by specifying that tagged changes like this remove any preceding tags with identical values (ie,
        // ensure that the change we report here becomes accurate after the fact)
        let relative_change = original_change.relative_to(&("one", "two").to_tree_address()).unwrap();

        // 'three', the first child of the 'two' node
        assert!(relative_change.applies_to(&"three".to_tree_address(), &TreeExtent::SubTree).unwrap());

        // 'five', the second child
        assert!(relative_change.applies_to(&"five".to_tree_address(), &TreeExtent::SubTree).unwrap());

        // Should be able to apply to the empty tree
        let empty_tree      = tree!("empty", "");
        let changed_tree    = relative_change.apply(&empty_tree);

        assert!(changed_tree.get_tag() == "two");
        assert!(changed_tree.get_child_at(0).get_tag() == "three");
    }
}
