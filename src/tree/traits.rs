use super::values::*;

///
/// The treenode trait is implemented by types that can act as part of a tree
///
pub trait TreeNode {
    ///
    /// Counts the number of children of this tree node
    ///
    fn count_children(&self) -> u32;

    ///
    /// Retrieves the child at the specified index
    ///
    fn get_child(&self, index: u32) -> &TreeNode;

    ///
    /// Retrieves the tag attached to this tree node
    ///
    fn get_tag(&self) -> &str;

    ///
    /// Retrieves the value attached to this node
    ///
    fn get_value(&self) -> &TreeValue;
}

///
/// This trait is implemented by types that can be converted into a tree node.
///
pub trait ToTreeNode {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Box<TreeNode>;
}

///
/// This trait is implemented by tree nodes that can be changed
///
pub trait MutableTreeNode : TreeNode {
    ///
    /// Adds a new child node to this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn add_child(&mut self, newNode: ToTreeNode) -> &MutableTreeNode;

    ///
    /// Removes the child node at the specified index. Returns the same node so many nodes can be altered as part of a single statement
    ///
    fn remove_child(&mut self, index: u32) -> &MutableTreeNode;

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value(&mut self, newValue: ToTreeValue) -> &MutableTreeNode;
}
