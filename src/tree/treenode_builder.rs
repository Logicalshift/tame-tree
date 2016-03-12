use std::rc::*;

use super::treenode::*;
use super::tree_ref::*;

pub trait TreeNodeBuilder {
    ///
    /// Sets the children of a tree node to a set of tree node references
    ///
    fn set_children_refs<'a>(&mut self, new_children: &'a Vec<Rc<TreeNode>>);
}

type ChildList = Rc<Vec<Rc<TreeNode>>>;

impl<T: MutableTreeNode> TreeNodeBuilder for T {
    fn set_children_refs<'a>(&mut self, new_children: &'a Vec<Rc<TreeNode>>) {
        let num_new_children = new_children.len();

        if num_new_children > 0 {
            // Build the final child node backwards
            let mut new_child = new_children[num_new_children-1].with_no_sibling();

            for child_num in (0..(num_new_children-1)).rev() {
                new_child = new_children[child_num].with_sibling_ref(&new_child);
            }

            self.set_child(new_child);
        } else {
            // No children
            self.clear_child();
        }
    }
}

#[cfg(test)]
mod treenode_builder_tests {
    use super::super::treenode::*;
    use super::super::basictree::*;

    #[test]
    fn can_build_tree() {
        let mut root = BasicTree::new("test", ());
        let child_list = vec!["child1".to_tree_node(), "child2".to_tree_node(), "child3".to_tree_node()];

        root.set_children_refs(&child_list);

        assert!(root.get_child_ref().is_some());
        assert!(root.get_child_ref_at(0).map(|x| x.get_tag() == "child1").unwrap_or(false));
        assert!(root.get_child_ref_at(1).map(|x| x.get_tag() == "child2").unwrap_or(false));
        assert!(root.get_child_ref_at(2).map(|x| x.get_tag() == "child3").unwrap_or(false));
        assert!(root.get_child_ref_at(3).is_none());
    }
}