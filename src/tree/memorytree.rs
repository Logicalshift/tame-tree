use super::traits::*;
use super::values::*;

type Node = Box<TreeNode>;

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

impl MemoryTree {
    ///
    /// Creates a new memory tree node, with a particular tag but no value
    ///
    pub fn new(tag: &str) -> MemoryTree {
        MemoryTree { tag: tag.to_string(), value: TreeValue::Nothing, child_nodes: Vec::<Node>::new() }
    }

    ///
    /// Adds a new child node to this tree
    ///
    pub fn add_child(&mut self, new_node: Box<TreeNode>) {
        self.child_nodes.push(new_node);
    }
}

#[cfg(test)]
mod memorytree_tests {
    use super::*;
    use tree::traits::*;

    #[test]
    fn can_add_child() {
        let mut tree = MemoryTree::new("root");
        let child_node = Box::new(MemoryTree::new("child"));

        tree.add_child(child_node);

        assert!(tree.count_children() == 1);
        assert!(tree.get_child(0).get_tag() == "child");
    }
}
