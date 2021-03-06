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

use std::rc::*;
use super::treenode::*;

///
/// Trait implemented by types that can work as a tree node index
///
pub trait TreeNodeIndex {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    fn lookup_index(&self, parent_node: &TreeRef) -> Option<TreeRef>;
}

impl TreeNodeIndex for usize {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    #[inline]
    fn lookup_index(&self, parent_node: &TreeRef) -> Option<TreeRef> {
        parent_node.lookup_child_at_index(*self)
    }
}

impl<'b> TreeNodeIndex for &'b str {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    /// When searching by tag, we match only the first item that we find.
    ///
    #[inline]
    fn lookup_index(&self, parent_node: &TreeRef) -> Option<TreeRef> {
        parent_node.lookup_child_with_tag(self)
    }
}

impl TreeNodeIndex for String {
    ///
    /// Finds the tree node corresponding to the specified index in the tree
    ///
    /// When searching by tag, we match only the first item that we find.
    ///
    #[inline]
    fn lookup_index(&self, parent_node: &TreeRef) -> Option<TreeRef> {
        (&**self).lookup_index(parent_node)
    }
}

///
/// Provides the ability to reference the children of a tree node by looking up a particular index
///
pub trait TreeNodeLookup {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> TreeRef;

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Option<TreeRef>;
}

impl<T: TreeNode + 'static> TreeNodeLookup for Rc<T> {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> TreeRef {
        let treenode: TreeRef  = self.to_owned();

        let opt_node = index.lookup_index(&treenode);
        let node_ref = opt_node.unwrap();

        node_ref
    }

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Option<TreeRef> {
        let treenode: TreeRef  = self.to_owned();

        index.lookup_index(&treenode)
    }
}

impl TreeNodeLookup for TreeRef {
    ///
    /// Looks up a child node at a particular index (panics if the child does not exist)
    ///
    fn get_child_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> TreeRef {
        let opt_node = index.lookup_index(self);
        let node_ref = opt_node.unwrap();

        node_ref
    }

    ///
    /// Looks up a child node at a particular index
    ///
    fn get_child_ref_at<TIndex: TreeNodeIndex>(&self, index: TIndex) -> Option<TreeRef> {
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
        let tree = Rc::new(BasicTree::new("test", (), Some("first_child".to_tree_node()), None));

        let tree_ref: TreeRef = tree.to_owned();
        let lookup = 0.lookup_index(&tree_ref);
        assert!(lookup.is_some());
        assert!(lookup.unwrap().get_tag() == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_get_first_child() {
        let tree = Rc::new(BasicTree::new("test", (), Some("first_child".to_tree_node()), None));

        assert!((tree.get_child_at(0).get_tag()) == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }

    #[test]
    fn can_get_first_child_by_string() {
        let tree = Rc::new(BasicTree::new("test", (), Some("first_child".to_tree_node()), None));

        assert!((tree.get_child_at("first_child").get_tag()) == "first_child");
        assert!(tree.get_sibling_ref().is_none());
    }
}
