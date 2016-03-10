use super::treenode::*;
use super::values::*;

///
/// Trait that provides some sugar functions that makes MutableTreeNode easier to use
///
pub trait MutableTreeNodeSugar : MutableTreeNode {
    ///
    /// Updates the child of this tree node
    ///
    fn set_child<TNode: ToTreeNode>(&mut self, new_node: TNode);

    ///
    /// Updates the sibling of this tree node
    ///
    fn set_sibling<TNode: ToTreeNode>(&mut self, new_node: TNode);

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue);
}

impl<T: MutableTreeNode> MutableTreeNodeSugar for T {
    ///
    /// Updates the child of this tree node
    ///
    fn set_child<TNode: ToTreeNode>(&mut self, new_node: TNode) {
        self.set_child_ref(new_node.to_tree_node());
    }

    ///
    /// Updates the sibling of this tree node
    ///
    fn set_sibling<TNode: ToTreeNode>(&mut self, new_node: TNode) {
        self.set_sibling_ref(new_node.to_tree_node());
    }

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue) {
        self.set_tree_value(new_value.to_tree_value());
    }
}
