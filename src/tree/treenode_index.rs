use std::rc::*;
use super::treenode::*;

///
/// Trait implemented by types that can work as a tree node index
///
pub trait TreeNodeIndex {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index(&self, parent_node: &Rc<TreeNode>) -> Option<Rc<TreeNode>>;
}

impl TreeNodeIndex for usize {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index(&self, parent_node: &Rc<TreeNode>) -> Option<Rc<TreeNode>> {
        let mut pos = *self;
        let mut current_child = parent_node.get_child_ref();

        loop {
            match current_child {
                None        => return None,
                Some(child) => {
                    if pos == 0 { 
                        return Some(child); 
                    }

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
    fn lookup_index(&self, parent_node: &Rc<TreeNode>) -> Option<Rc<TreeNode>> {
        let mut current_child = parent_node.get_child_ref().to_owned();

        loop {
            match current_child {
                None        => return None,
                Some(child) => {
                    if child.get_tag() == *self {
                        return Some(child);
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
pub trait TreeNodeLookup {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Rc<TreeNode>;

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Option<Rc<TreeNode>>;
}

impl<T: TreeNode + 'static> TreeNodeLookup for Rc<T> {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Rc<TreeNode> {
        let treenode: Rc<TreeNode>  = self.to_owned();

        let opt_node = index.lookup_index(&treenode);
        let node_ref = opt_node.unwrap();

        node_ref
    }

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Option<Rc<TreeNode>> {
        let treenode: Rc<TreeNode>  = self.to_owned();

        index.lookup_index(&treenode)
    }
}

impl TreeNodeLookup for Rc<TreeNode> {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Rc<TreeNode> {
        let opt_node = index.lookup_index(self);
        let node_ref = opt_node.unwrap();

        node_ref
    }

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Option<Rc<TreeNode>> {
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
        let tree = Rc::new(BasicTree::new("test", ()));
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        let tree_ref: Rc<TreeNode> = tree.to_owned();
        let lookup = 0.lookup_index(&tree_ref);
        assert!(lookup.is_some());
        assert!(lookup.unwrap().get_tag() == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_get_first_child() {
        let tree = Rc::new(BasicTree::new("test", ()));
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        assert!((tree.get_child_at(0).get_tag()) == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_get_first_child_by_string() {
        let tree = Rc::new(BasicTree::new("test", ()));
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        assert!((tree.get_child_at("first_child").get_tag()) == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }
}
