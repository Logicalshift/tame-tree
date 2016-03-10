use super::treenode::*;
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
    /// Retrieves a reference to the child at the specified index
    ///
    fn get_child_ref(&self, index: u32) -> &Rc<TreeNode> {
        &(self.child_nodes[index as usize])
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
    fn add_child_ref(&mut self, new_node: Rc<TreeNode>, at_index: u32) -> &mut MutableTreeNode {
        self.child_nodes.insert(at_index as usize, new_node);
        self
    }

    ///
    /// Replaces a child node with a different one
    ///
    fn replace_child_ref(&mut self, replacement_node: Rc<TreeNode>, at_index: u32) -> &mut MutableTreeNode {
        self.child_nodes[at_index as usize] = replacement_node;
        self
    }

    ///
    /// Removes a node from this tree
    ///
    fn remove_child(&mut self, index: u32) -> &mut MutableTreeNode {
        self.child_nodes.remove(index as usize);
        self
    }

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_tree_value(&mut self, new_value: TreeValue) -> &mut MutableTreeNode {
        self.value = new_value;
        self
    }

    ///
    /// Returns a reference to a mutable version of a particular child node
    ///
    fn alter_child(&mut self, at_index: u32) -> &mut MutableTreeNode {
        // Try to get the child as a mutable reference
        let array_index = at_index as usize;
        let mutable_child = Rc::get_mut(self.child_nodes[array_index]);

        // The child may be referenced in multiple places
        match mutable_child {
            // We'll get a mutable result if we're the sole owner
            Some(child) => return child,

            // None indicates that the child is shared, so we need to create a copy
            None => {
                // Generate a copy of this child
                self.child_nodes[array_index] = MemoryTree::from(self.child_nodes[array_index]);

                return Rc::get_mut(&mut self.child_nodes[array_index]).unwrap();
            }
        }
    }
}

impl Clone for MemoryTree {
    fn clone(&self) -> MemoryTree {
        MemoryTree { tag: self.tag.to_owned(), value: self.value.to_owned(), child_nodes: self.child_nodes.to_owned() }
    }
}

impl MemoryTree {
    ///
    /// Creates a new memory tree node, with a particular tag but no value
    ///
    pub fn new<TValue: ToTreeValue>(tag: &str, value: TValue) -> MemoryTree {
        MemoryTree { tag: tag.to_string(), value: value.to_tree_value(), child_nodes: Vec::<Node>::new() }
    }

    ///
    /// Creates a new tree node from an existing node
    ///
    pub fn from<TNode: ToTreeNode>(node: TNode) -> MemoryTree {
        let as_tree_node = node.to_tree_node();

        // Generate the node
        let mut result = MemoryTree::new(as_tree_node.get_tag(), as_tree_node.get_value());

        // Copy (well, reference) the other node's child nodes into this one
        for child_id in 0..as_tree_node.count_children() {
            result.add_child_ref(as_tree_node.get_child_ref(child_id).to_tree_node(), child_id);
        }

        result
    }
}

#[cfg(test)]
mod memorytree_tests {
    use super::*;
    use super::super::treenode::*;

    #[test]
    fn can_add_child() {
        let mut tree = MemoryTree::new("root", ());
        let child_node = MemoryTree::new("child", ());

        tree.add_child(child_node, 0);

        assert!(tree.count_children() == 1);
        assert!(tree.get_child(0).get_tag() == "child");
    }
}
