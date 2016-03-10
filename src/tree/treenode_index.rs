use std::rc::*;
use super::treenode::*;

///
/// Trait implemented by types that can work as a tree node index
///
pub trait TreeNodeIndex : Sized {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index<'a, T: TreeNode>(&'a self, parent_node: &'a T) -> Option<&Rc<TreeNode>>;
}

impl TreeNodeIndex for usize {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index<'a, T: TreeNode>(&'a self, parent_node: &'a T) -> Option<&Rc<TreeNode>> {
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

///
/// Provides the ability to reference the children of a tree node by looking up a particular index
///
pub trait TreeNodeLookup : TreeNode {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> &TreeNode;
}

impl<T: TreeNode> TreeNodeLookup for T {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> &TreeNode {
        let opt_node = index.lookup_index(self);
        let node_ref = opt_node.unwrap();

        &**node_ref
    }
}

/*
impl<TIndex: TreeNodeIndex> Index<TIndex> for TreeNode {
    type Output = TreeNode;

    fn index<'a>(&'a self, index: TIndex) -> &'a TreeNode {
        let opt_node = index.lookup_index(self);
        let node_ref = opt_node.unwrap();

        &**node_ref
    }
}
*/

#[cfg(test)]
mod treenode_index_tests {
    use super::super::values::*;
    use super::super::treenode::*;
    use super::super::basictree::*;
    use std::rc::*;

    #[test]
    fn can_get_first_child() {
        let mut tree = BasicTree::new("test", ());
        let first_child = Rc::new(BasicTree::new("first_child", ()));

        tree.set_child_ref(first_child);

        assert!(tree.get_child_at(0).is_some());
        assert!((tree.get_child_at(0).unwrap().get_tag()) == "child");
        assert!(tree.get_sibling_ref().is_none());
    }
}