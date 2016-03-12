use std::rc::*;

use super::treenode::*;
use super::values::*;

///
/// A TreeRef works as a reference to another tree node with a different sibling
///
pub struct TreeRef {
    ref_to: Rc<TreeNode>,
    new_sibling: Option<Rc<TreeNode>>
}

impl TreeNode for TreeRef {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<&Rc<TreeNode>> {
        (*self.ref_to).get_child_ref()
    }

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<&Rc<TreeNode>> {
        self.new_sibling.as_ref()
    }

    ///
    /// Retrieves the tag attached to this tree node
    ///
    fn get_tag(&self) -> &str {
        (*self.ref_to).get_tag()
    }

    ///
    /// Retrieves the value attached to this node
    ///
    fn get_value(&self) -> &TreeValue {
        (*self.ref_to).get_value()
    }
}

///
/// Trait provided by types that can generate treenodes with new siblings
///
pub trait ToTreeRef {
    ///
    /// Creates a new tree node that is identical to an existing one apart from its sibling
    ///
    fn with_sibling_ref(&self, new_sibling: &Rc<TreeNode>) -> Rc<TreeNode>;

    ///
    /// Creates a new tree node that is identical to an existing one apart from having no sibling
    ///
    fn with_no_sibling_ref(&self) -> Rc<TreeNode>;
}

impl ToTreeRef for Rc<TreeNode> {
    ///
    /// Creates a new tree node that is identical to an existing one apart from its sibling
    ///
    fn with_sibling_ref(&self, new_sibling: &Rc<TreeNode>) -> Rc<TreeNode> {
        Rc::new(TreeRef { ref_to: self.to_owned(), new_sibling: Some(new_sibling.to_owned()) })
    }

    ///
    /// Creates a new tree node that is identical to an existing one apart from having no sibling
    ///
    fn with_no_sibling_ref(&self) -> Rc<TreeNode> {
        Rc::new(TreeRef { ref_to: self.to_owned(), new_sibling: None })
    }
}

///
/// Trait providing 'sugar' functions for calling ToTreeRef style funtions on things implementing ToTreeNode
///
pub trait ToTreeRefSugar {
    ///
    /// Creates a new tree node with a particular sibling
    ///
    fn with_sibling<TSibling: ToTreeNode>(&self, new_sibling: TSibling) -> Rc<TreeNode>;

    ///
    /// Creates a new tree node with no sibling
    ///
    fn with_no_sibling(&self) -> Rc<TreeNode>;
}

impl<TNode: ToTreeNode> ToTreeRefSugar for TNode {
    ///
    /// Creates a new tree node with a particular sibling
    ///
    fn with_sibling<TSibling: ToTreeNode>(&self, new_sibling: TSibling) -> Rc<TreeNode> {
        let sibling_node = new_sibling.to_tree_node();
        self.to_tree_node().with_sibling_ref(&sibling_node)
    }

    ///
    /// Creates a new tree node with no sibling
    ///
    fn with_no_sibling(&self) -> Rc<TreeNode> {
        self.to_tree_node().with_no_sibling_ref()
    }
}

#[cfg(test)]
mod tree_ref_tests {
    use super::*;
    use super::super::treenode::*;

    #[test]
    fn create_with_sibling() {
        let root            = "root".to_tree_node();
        let with_sibling    = root.with_sibling("sibling");
        let no_sibling      = with_sibling.with_no_sibling();

        assert!(root.get_tag() == "root");
        assert!(root.get_sibling_ref().is_none());
        assert!(with_sibling.get_tag() == "root");
        assert!(!with_sibling.get_sibling_ref().is_none());
        assert!(no_sibling.get_sibling_ref().is_none());
    }
}
