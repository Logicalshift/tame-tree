use std::rc::*;

use super::address::*;
use super::extent::*;
use super::treenode::*;

///
/// Represents which of the root's references have changed
///
pub enum TreeChangeType {
    /// The node's child reference has been replaced
    Child,

    /// The node's sibling reference has been replaced
    Sibling,
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

    /// The extent of the nodes being replaced (starting at the node currently at the replacement reference)
    replacement_range: TreeExtent,

    /// The tree that should replace the changed reference. The last node in this tree (depth-first) will be given the same sibling as the last node in the replacement range
    replacement_tree: Rc<TreeNode>
}

impl TreeChange {
    /// Creates a new tree change
    pub fn new<TAddress: ToTreeAddress, TNode: ToTreeNode>(root: &TAddress, change_type: TreeChangeType, replacement_range: TreeExtent, replacement_tree: &TNode) -> TreeChange {
        TreeChange { root: root.to_tree_address(), change_type: change_type, replacement_range: replacement_range, replacement_tree: replacement_tree.to_tree_node() }
    }
}
