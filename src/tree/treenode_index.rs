use std::rc::*;
use super::treenode::*;

///
/// Trait implemented by types that can work as a tree node index
///
trait TreeNodeIndex {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index<'a>(&'a self, parent_node: &'a TreeNode) -> Option<&Rc<TreeNode>>;
}

impl TreeNodeIndex for usize {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index<'a>(&'a self, parent_node: &'a TreeNode) -> Option<&Rc<TreeNode>> {
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