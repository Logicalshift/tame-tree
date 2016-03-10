use super::values::*;
use std::rc::*;

///
/// The treenode trait is implemented by types that can act as part of a tree
///
pub trait TreeNode {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<&Rc<TreeNode>>;

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<&Rc<TreeNode>>;

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
    fn to_tree_node(&self) -> Rc<TreeNode>;
}

///
/// This trait is implemented by tree nodes that can be changed
///
pub trait MutableTreeNode : TreeNode {
    ///
    /// Sets the child for this tree node
    ///
    fn set_child_ref(&mut self, new_node: Rc<TreeNode>);

    ///
    /// Sets the sibling for this tree node
    ///
    fn set_sibling_ref(&mut self, new_node: Rc<TreeNode>);

    ///
    /// Unsets the child for this node
    ///
    fn clear_child(&mut self);

    ///
    /// Unsets the sibling for this node
    ///
    fn clear_sibling(&mut self);

    ///
    /// Changes the value set for this node.
    ///
    fn set_tree_value(&mut self, new_value: TreeValue);

    ///
    /// Changes the tag attached to this tree
    ///
    fn set_tag(&mut self, new_tag: &str);
}

/*
///
/// Trait that provides some sugar functions that makes MutableTreeNode easier to use
///
pub trait MutableTreeNodeSugar : MutableTreeNode {
    ///
    /// Adds a new child node to this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn add_child<TNode: ToTreeNode>(&mut self, new_node: TNode, at_index: u32) -> &mut MutableTreeNode;

    ///
    /// Replaces a child node with a different one
    ///
    fn replace_child<TNode: ToTreeNode>(&mut self, replacement_node: TNode, at_index: u32) -> &mut MutableTreeNode;

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue) -> &mut MutableTreeNode;
}

impl<T: MutableTreeNode> MutableTreeNodeSugar for T {
    ///
    /// Adds a new child node to this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn add_child<TNode: ToTreeNode>(&mut self, new_node: TNode, at_index: u32) -> &mut MutableTreeNode {
        self.add_child_ref(new_node.to_tree_node(), at_index);
        self
    }

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue) -> &mut MutableTreeNode {
        self.set_tree_value(new_value.to_tree_value());
        self
    }

    ///
    /// Replaces a child node with a different one
    ///
    fn replace_child<TNode: ToTreeNode>(&mut self, replacement_node: TNode, at_index: u32) -> &mut MutableTreeNode {
        self.replace_child_ref(replacement_node.to_tree_node(), at_index);
        self
    }
}
*/

impl<T> ToTreeNode for T where T: TreeNode, T: Clone, T: 'static {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        Rc::new(self.clone())
    }
}

impl ToTreeNode for Rc<TreeNode> {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        self.clone()
    }
}