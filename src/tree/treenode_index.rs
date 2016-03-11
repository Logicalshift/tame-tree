use std::rc::*;
use super::treenode::*;

///
/// Trait implemented by types that can work as a tree node index
///
pub trait TreeNodeIndex {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index<'a>(&self, parent_node: &'a TreeNode) -> Option<&'a Rc<TreeNode>>;
}

impl TreeNodeIndex for usize {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index<'a>(&self, parent_node: &'a TreeNode) -> Option<&'a Rc<TreeNode>> {
        let mut pos = *self;
        let mut current_child = parent_node.get_child_ref().to_owned();

        loop {
            match current_child {
                None        => return None,
                Some(child) => {
                    if pos == 0 { return current_child; }

                    pos = pos-1;
                    current_child = child.get_sibling_ref().to_owned();
                }
            }
        }
    }
}

impl<'b> TreeNodeIndex for &'b str {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    /// When searching by tag, we match only the first item that we find.
    ///
    fn lookup_index<'a>(&self, parent_node: &'a TreeNode) -> Option<&'a Rc<TreeNode>> {
        let mut current_child = parent_node.get_child_ref().to_owned();

        loop {
            match current_child {
                None        => return None,
                Some(child) => {
                    if (*child).get_tag() == *self {
                        return current_child;
                    }

                    current_child = child.get_sibling_ref().to_owned();
                }
            }
        }
    }
}

///
/// Provides the ability to reference the children of a tree node by looking up a particular index
///
pub trait TreeNodeLookup : TreeNode {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<'a, TIndex: TreeNodeIndex>(&'a self, index: TIndex) -> &'a TreeNode;

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<'a, TIndex: TreeNodeIndex>(&'a self, index: TIndex) -> Option<&'a Rc<TreeNode>>;
}

impl<T: TreeNode> TreeNodeLookup for T {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<'a, TIndex: TreeNodeIndex>(&'a self, index: TIndex) -> &'a TreeNode {
        let opt_node = index.lookup_index(self);
        let node_ref = opt_node.unwrap();

        &**node_ref
    }

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<'a, TIndex: TreeNodeIndex>(&'a self, index: TIndex) -> Option<&'a Rc<TreeNode>> {
        index.lookup_index(self)
    }
}

#[cfg(test)]
mod treenode_index_tests {
    use super::super::treenode::*;
    use super::super::basictree::*;
    use std::rc::*;

    #[test]
    fn lookup_usize() {
        let mut tree = BasicTree::new("test", ());
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        let lookup = 0.lookup_index(&tree);
        assert!(lookup.is_some());
        assert!(lookup.unwrap().get_tag() == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_get_first_child() {
        let mut tree = BasicTree::new("test", ());
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        assert!((tree.get_child_at(0).get_tag()) == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_get_first_child_by_string() {
        let mut tree = BasicTree::new("test", ());
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        assert!((tree.get_child_at("first_child").get_tag()) == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }
}
