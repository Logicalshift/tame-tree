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

use super::treenode::*;
use super::values::*;
use std::rc::*;

///
/// BasicTree is a basic in-memory tree node
///
pub struct BasicTree {
    tag: String,
    value: TreeValue,

    child: Option<TreeRef>,
    sibling: Option<TreeRef>
}

impl BasicTree {
    ///
    /// Creates a new tree node with a particular tag and no siblings
    ///
    pub fn new<TValue: ToTreeValue>(tag: &str, value: TValue, child: Option<TreeRef>, sibling: Option<TreeRef>) -> BasicTree {
        BasicTree { tag: tag.to_string(), value: value.to_tree_value(), child: child, sibling: sibling }
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

    ///
    /// Copies a node into a new basic node and replaces the references
    ///
    pub fn from_with_references<TNode: ToTreeNode>(node: TNode, new_child: Option<&TreeRef>, new_sibling: Option<&TreeRef>) -> BasicTree {
        let as_tree_node    = node.to_tree_node();

        BasicTree { 
            tag:        as_tree_node.get_tag().to_owned(), 
            value:      as_tree_node.get_value().to_owned(), 
            child:      new_child.map(|x| { x.clone() }),
            sibling:    new_sibling.map(|x| { x.clone() })
        }
    }

    ///
    /// Copies a node into a new basic node and replaces the child (the sibling is preserved)
    ///
    pub fn from_with_child<TNode: ToTreeNode>(node: TNode, new_child: TreeRef) -> BasicTree {
        let as_tree_node    = node.to_tree_node();
        let sibling         = as_tree_node.get_sibling_ref();

        BasicTree { 
            tag:        as_tree_node.get_tag().to_owned(), 
            value:      as_tree_node.get_value().to_owned(), 
            child:      Some(new_child),
            sibling:    sibling
        }
    }

    ///
    /// Copies a node into a new basic node and replaces the sibling (the child is preserved)
    ///
    pub fn from_with_sibling<TNode: ToTreeNode>(node: TNode, new_sibling: TreeRef) -> BasicTree {
        let as_tree_node    = node.to_tree_node();
        let child           = as_tree_node.get_child_ref();

        BasicTree { 
            tag:        as_tree_node.get_tag().to_owned(), 
            value:      as_tree_node.get_value().to_owned(), 
            child:      child,
            sibling:    Some(new_sibling)
        }
    }
}

impl TreeNode for BasicTree {
    ///
    /// Retrieves a reference to the child of this tree node (or None if this node has no child)
    ///
    fn get_child_ref(&self) -> Option<TreeRef> {
        self.child.clone()
    }

    ///
    /// Retrieves a reference to the sibling of this tree node (or None if this node has no sibling)
    ///
    fn get_sibling_ref(&self) -> Option<TreeRef> {
        self.sibling.clone()
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

    ///
    /// Creates a copy of this node with different references
    ///
    #[inline]
    fn with_references(&self, new_child: Option<&TreeRef>, new_sibling: Option<&TreeRef>) -> TreeRef {
        Rc::new(BasicTree::new(&*self.tag, self.value.clone(), new_child.map(|x| { x.clone() }), new_sibling.map(|x| { x.clone() })))
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
    fn to_tree_node(&self) -> TreeRef {
        Rc::new(BasicTree::new(self, (), None, None))
    }
}

impl<'a, TValue: ToTreeValue> ToTreeNode for (&'a str, TValue) {
    fn to_tree_node(&self) -> TreeRef {
        let (ref tag, ref value) = *self;
        Rc::new(BasicTree::new(tag, value.to_tree_value(), None, None))
    }
}

#[cfg(test)]
mod basictree_tests {
    use super::*;
    use super::super::treenode::*;

    #[test]
    fn can_create_basictree() {
        let tree = BasicTree::new("test", (), None, None);

        assert!(tree.get_tag() == "test");
        assert!(tree.get_value().is_nothing());
        assert!(tree.get_child_ref().is_none());
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn value_is_set() {
        let tree = BasicTree::new("test", 1, None, None);

        assert!(tree.get_tag() == "test");
        assert!(!tree.get_value().is_nothing());
    }

    #[test]
    fn can_set_child() {
        let tree = BasicTree::new("test", (), Some(("child", "childvalue").to_tree_node()), None);

        assert!(tree.get_child_ref().is_some());
        assert!((tree.get_child_ref().unwrap().get_tag()) == "child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_set_sibling() {
        let tree = BasicTree::new("test", (), None, Some(("sibling", ()).to_tree_node()));

        assert!(tree.get_sibling_ref().is_some());
        assert!((tree.get_sibling_ref().unwrap().get_tag()) == "sibling");
        assert!(tree.get_child_ref().is_none());
    }

    #[test]
    fn can_clone_from() {
        let tree = "tree".to_tree_node();
        let copy = BasicTree::from(tree);

        assert!(copy.get_tag() == "tree");
    }
}
