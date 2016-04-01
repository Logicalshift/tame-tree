//
//   Copyright 2016 Andrew Hunter
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
//

use super::values::*;
use std::rc::*;

pub use super::treenode_index::*;
pub use super::treenode_builder::*;

///
/// Reference to a tree node
///
pub type TreeRef = Rc<TreeNode>;

///
/// The treenode trait is implemented by types that can act as part of a tree
///
pub trait TreeNode {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<TreeRef>;

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<TreeRef>;

    ///
    /// Retrieves the tag attached to this tree node
    ///
    fn get_tag(&self) -> &str;

    ///
    /// Retrieves the value attached to this node
    ///
    fn get_value(&self) -> &TreeValue;

    ///
    /// Creates a copy of this node with different references
    ///
    fn with_references(&self, new_child: Option<&TreeRef>, new_sibling: Option<&TreeRef>) -> TreeRef;

    ///
    /// Creates a copy of this node with a specific child node
    ///
    #[inline]
    fn with_child_node(&self, new_child: Option<&TreeRef>) -> TreeRef {
        self.with_references(new_child, self.get_sibling_ref().as_ref())
    }

    ///
    /// Creates a copy of this node with a specific sibling node
    ///
    #[inline]
    fn with_sibling_node(&self, new_sibling: Option<&TreeRef>) -> TreeRef {
        self.with_references(self.get_child_ref().as_ref(), new_sibling)
    }

    ///
    /// Creates a copy of this node with a specific set of child nodes
    ///
    fn with_children(&self, new_children: &Vec<TreeRef>) -> TreeRef {
        let mut new_child = None;

        for sibling in new_children.into_iter().rev() {
            new_child = Some(sibling.with_sibling_node(new_child.as_ref()));
        }

        self.with_child_node(new_child.as_ref())
    }

    ///
    /// Looks up the child at the specified index
    ///
    fn lookup_child_at_index(&self, index: usize) -> Option<TreeRef> {
        let mut result = self.get_child_ref();

        for _ in 0..index {
            result = result.and_then(|x| { x.get_sibling_ref() });
        }

        result
    }

    ///
    /// Looks up the child at the specified index
    ///
    fn lookup_child_with_tag(&self, tag: &str) -> Option<TreeRef> {
        let mut current = self.get_child_ref();

        loop {
            match current {
                None        => { return None; },
                Some(node) => {
                    if node.get_tag() == tag {
                        return Some(node);
                    } else {
                        current = node.get_sibling_ref();
                    }
                }
            }
        }
    }
}

///
/// This trait is implemented by types that can be converted into a tree node.
///
pub trait ToTreeNode {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> TreeRef;
}

impl<T> ToTreeNode for Rc<T> where T: TreeNode, T: 'static {
    ///
    /// Converts this value into a tree node
    ///
    #[inline]
    fn to_tree_node(&self) -> TreeRef {
        self.clone()
    }
}

impl ToTreeNode for TreeRef {
    ///
    /// Converts this value into a tree node
    ///
    #[inline]
    fn to_tree_node(&self) -> TreeRef {
        self.clone()
    }
}

impl<'a, T> ToTreeNode for &'a Rc<T> where T: TreeNode, T: 'static {
    ///
    /// Converts this value into a tree node
    ///
    #[inline]
    fn to_tree_node(&self) -> TreeRef {
        (*self).clone()
    }
}


impl<'a> ToTreeNode for &'a TreeRef {
    ///
    /// Converts this value into a tree node
    ///
    #[inline]
    fn to_tree_node(&self) -> TreeRef {
        (*self).clone()
    }
}

impl TreeNode for TreeRef {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    #[inline]
    fn get_child_ref(&self) -> Option<TreeRef> {
        (**self).get_child_ref()
    }

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    #[inline]
    fn get_sibling_ref(&self) -> Option<TreeRef> {
        (**self).get_sibling_ref()
    }

    ///
    /// Retrieves the tag attached to this tree node
    ///
    #[inline]
    fn get_tag(&self) -> &str {
        (**self).get_tag()
    }

    ///
    /// Retrieves the value attached to this node
    ///
    #[inline]
    fn get_value(&self) -> &TreeValue {
        (**self).get_value()
    }

    ///
    /// Creates a copy of this node with different references
    ///
    #[inline]
    fn with_references(&self, new_child: Option<&TreeRef>, new_sibling: Option<&TreeRef>) -> TreeRef {
        (**self).with_references(new_child, new_sibling)
    }

    ///
    /// Creates a copy of this node with a specific child node
    ///
    #[inline]
    fn with_child_node(&self, new_child: Option<&TreeRef>) -> TreeRef {
        (**self).with_child_node(new_child)
    }

    ///
    /// Creates a copy of this node with a specific sibling node
    ///
    #[inline]
    fn with_sibling_node(&self, new_sibling: Option<&TreeRef>) -> TreeRef {
        (**self).with_sibling_node(new_sibling)
    }

    ///
    /// Creates a copy of this node with a specific set of child nodes
    ///
    #[inline]
    fn with_children(&self, new_children: &Vec<TreeRef>) -> TreeRef {
        (**self).with_children(new_children)
    }

    ///
    /// Looks up the child at the specified index
    ///
    #[inline]
    fn lookup_child_at_index(&self, index: usize) -> Option<TreeRef> {
        (**self).lookup_child_at_index(index)
    }

    ///
    /// Looks up the child at the specified index
    ///
    #[inline]
    fn lookup_child_with_tag(&self, tag: &str) -> Option<TreeRef> {
        (**self).lookup_child_with_tag(tag)
    }
}
