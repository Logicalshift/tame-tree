use super::values::*;
use std::rc::*;

pub use super::treenode_sugar::*;
pub use super::treenode_index::*;
pub use super::treenode_builder::*;

///
/// The treenode trait is implemented by types that can act as part of a tree
///
pub trait TreeNode {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<Rc<TreeNode>>;

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<Rc<TreeNode>>;

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
    fn set_child_ref(&self, new_node: Rc<TreeNode>);

    ///
    /// Sets the sibling for this tree node
    ///
    fn set_sibling_ref(&self, new_node: Rc<TreeNode>);

    ///
    /// Unsets the child for this node
    ///
    fn clear_child(&self);

    ///
    /// Unsets the sibling for this node
    ///
    fn clear_sibling(&self);

    ///
    /// Changes the value set for this node.
    ///
    fn set_tree_value(&mut self, new_value: TreeValue);

    ///
    /// Changes the tag attached to this tree
    ///
    fn set_tag(&mut self, new_tag: &str);
}

impl<T> ToTreeNode for Rc<T> where T: TreeNode, T: 'static {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        self.clone()
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

impl<'a, T> ToTreeNode for &'a Rc<T> where T: TreeNode, T: 'static {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        (*self).clone()
    }
}


impl<'a> ToTreeNode for &'a Rc<TreeNode> {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        (*self).clone()
    }
}

impl TreeNode for Rc<TreeNode> {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<Rc<TreeNode>> {
        return (**self).get_child_ref();
    }

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<Rc<TreeNode>> {
        return (**self).get_sibling_ref();
    }

    ///
    /// Retrieves the tag attached to this tree node
    ///
    fn get_tag(&self) -> &str {
        return (**self).get_tag();
    }

    ///
    /// Retrieves the value attached to this node
    ///
    fn get_value(&self) -> &TreeValue {
        return (**self).get_value();
    }
}
