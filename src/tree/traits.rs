use super::values::*;

///
/// The treenode trait is implemented by things that can act as part of a tree
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
