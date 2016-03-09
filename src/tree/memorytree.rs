use super::traits::*;
use super::values::*;
use std::rc::*;

type Node = Rc<TreeNode>;

///
/// A tree whose structure is kept in-memory and which has no pre-defined structure
///
pub struct MemoryTree {
    child_nodes: Vec<Node>,
    tag: String,
    value: TreeValue
}

impl TreeNode for MemoryTree {
    ///
    /// Counts the number of children of this tree node
    ///
    fn count_children(&self) -> u32 {
        self.child_nodes.len() as u32
    }

    ///
    /// Retrieves the child at the specified index
    ///
    fn get_child(&self, index: u32) -> &TreeNode {
        &*self.child_nodes[index as usize]
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

impl MutableTreeNode for MemoryTree {
    ///
    /// Adds a new child node to this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn add_child<TNode: ToTreeNode>(&mut self, new_node: TNode, at_index: u32) -> &mut MemoryTree {
        self.child_nodes.insert(at_index as usize, new_node.to_tree_node());
        self
    }

    ///
    /// Removes a node from this tree
    ///
    fn remove_child(&mut self, index: u32) -> &mut MemoryTree {
        self.child_nodes.remove(index as usize);
        self
    }

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue) -> &mut MemoryTree {
        self.value = new_value.to_tree_value();
        self
    }
}

impl Clone for MemoryTree {
    fn clone(&self) -> MemoryTree {
        MemoryTree { tag: self.tag.to_owned(), value: self.value.to_owned(), child_nodes: self.child_nodes.to_owned() }
    }
}

impl ToTreeNode for MemoryTree {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        Rc::new(self.to_owned())
    }
}

impl MemoryTree {
    ///
    /// Creates a new memory tree node, with a particular tag but no value
    ///
    pub fn new<TValue: ToTreeValue>(tag: &str, value: TValue) -> MemoryTree {
        MemoryTree { tag: tag.to_string(), value: value.to_tree_value(), child_nodes: Vec::<Node>::new() }
    }
}

#[cfg(test)]
mod memorytree_tests {
    use super::*;
    use tree::traits::*;

    #[test]
    fn can_add_child() {
        let mut tree = MemoryTree::new("root", ());
        let child_node = MemoryTree::new("child", ());

        tree.add_child(child_node, 0);

        assert!(tree.count_children() == 1);
        assert!(tree.get_child(0).get_tag() == "child");
    }
}
