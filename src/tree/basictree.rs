use super::treenode::*;
use super::values::*;
use std::rc::*;

///
/// BasicTree is a basic in-memory tree node
///
pub struct BasicTree {
    tag: String,
    value: TreeValue,

    child: Option<Rc<TreeNode>>,
    sibling: Option<Rc<TreeNode>>
}

impl BasicTree {
    ///
    /// Creates a new tree node with a particular tag and no siblings
    ///
    pub fn new<TValue: ToTreeValue>(tag: &str, value: TValue) -> BasicTree {
        BasicTree { tag: tag.to_string(), value: value.to_tree_value(), child: None, sibling: None }
    }

    ///
    /// Copies a node into a new basic node
    ///
    pub fn from<TNode: ToTreeNode>(node: TNode) -> BasicTree {
        let as_tree_node    = node.to_tree_node();
        let child           = as_tree_node.get_child_ref();
        let sibling         = as_tree_node.get_sibling_ref();

        BasicTree { 
            tag:        as_tree_node.get_tag().to_owned(), 
            value:      as_tree_node.get_value().to_owned(), 
            child:      child,
            sibling:    sibling
        }
    }
}

impl TreeNode for BasicTree {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<Rc<TreeNode>> {
        match self.child {
            Some(ref child) => Some(child.to_owned()),
            None => None
        }
    }

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<Rc<TreeNode>> {
        match self.sibling {
            Some(ref sibling) => Some(sibling.to_owned()),
            None => None
        }
    }

    ///
    /// Retrieves the tag attached to this tree node
    ///
    fn get_tag(&self) -> &str {
        &self.tag
    }

    ///
    /// Retrieves the value attached to this node
    ///
    fn get_value(&self) -> &TreeValue {
        &self.value
    }
}

impl MutableTreeNode for BasicTree {
    ///
    /// Sets the child for this tree node
    ///
    fn set_child_ref(&mut self, new_node: Rc<TreeNode>) {
        self.child = Some(new_node);
    }

    ///
    /// Sets the sibling for this tree node
    ///
    fn set_sibling_ref(&mut self, new_node: Rc<TreeNode>) {
        self.sibling = Some(new_node);
    }

    ///
    /// Unsets the child for this node
    ///
    fn clear_child(&mut self) {
        self.child = None;
    }

    ///
    /// Unsets the sibling for this node
    ///
    fn clear_sibling(&mut self) {
        self.sibling = None;
    }

    ///
    /// Changes the value set for this node.
    ///
    fn set_tree_value(&mut self, new_value: TreeValue) {
        self.value = new_value;
    }

    ///
    /// Changes the tag attached to this tree
    ///
    fn set_tag(&mut self, new_tag: &str) {
        self.tag = new_tag.to_string();
    }
}

impl Clone for BasicTree {
    fn clone(&self) -> BasicTree {
        BasicTree { 
            tag:        self.tag.to_owned(), 
            value:      self.value.to_owned(), 
            child:      self.child.to_owned(),
            sibling:    self.sibling.to_owned() }
    }
}

impl<'a> ToTreeNode for &'a str {
    fn to_tree_node(&self) -> Rc<TreeNode> {
        Rc::new(BasicTree::new(self, ()))
    }
}

impl<'a, TValue: ToTreeValue> ToTreeNode for (&'a str, TValue) {
    fn to_tree_node(&self) -> Rc<TreeNode> {
        let (ref tag, ref value) = *self;
        Rc::new(BasicTree::new(tag, value.to_tree_value()))
    }
}

#[cfg(test)]
mod basictree_tests {
    use super::*;
    use super::super::values::*;
    use super::super::treenode::*;
    use std::rc::*;

    #[test]
    fn can_create_basictree() {
        let tree = BasicTree::new("test", ());

        assert!(tree.get_tag() == "test");
        assert!(tree.get_value().is_nothing());
        assert!(tree.get_child_ref().is_none());
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn value_is_set() {
        let tree = BasicTree::new("test", 1);

        assert!(tree.get_tag() == "test");
        assert!(!tree.get_value().is_nothing());
    }

    #[test]
    fn can_set_child() {
        let mut tree = BasicTree::new("test", ());

        tree.set_child(("child", "childvalue"));

        assert!(tree.get_child_ref().is_some());
        assert!((tree.get_child_ref().unwrap().get_tag()) == "child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_set_sibling() {
        let mut tree = BasicTree::new("test", ());
        let sibling = Rc::new(BasicTree::new("sibling", ()));

        tree.set_sibling_ref(sibling);

        assert!(tree.get_sibling_ref().is_some());
        assert!((tree.get_sibling_ref().unwrap().get_tag()) == "sibling");
        assert!(tree.get_child_ref().is_none());
    }

    #[test]
    fn can_set_tag() {
        let mut tree = BasicTree::new("test", ());

        tree.set_tag("newtag");

        assert!(tree.get_tag() == "newtag");
    }

    #[test]
    fn can_set_value() {
        let mut tree = BasicTree::new("test", ());

        assert!(tree.get_value().is_nothing());

        tree.set_tree_value(TreeValue::String("Some value".to_string()));

        assert!(match *tree.get_value() { TreeValue::String(ref x) => x == "Some value", _ => false });
    }

    #[test]
    fn can_clone_from() {
        let tree = "tree".to_tree_node();
        let copy = BasicTree::from(tree);

        assert!(copy.get_tag() == "tree");
    }
}
