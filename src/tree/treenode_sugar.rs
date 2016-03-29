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

///
/// Sugar functions for MutableTreeNode: various functions that can be implemented in terms of
/// the standard mutable tree node functions.
///
pub trait MutableTreeNodeSugar {
    ///
    /// Updates the child of this tree node
    ///
    fn set_child<TNode: ToTreeNode>(&self, new_node: TNode);

    ///
    /// Updates the sibling of this tree node
    ///
    fn set_sibling<TNode: ToTreeNode>(&self, new_node: TNode);

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue);
}

impl<T: MutableTreeNode> MutableTreeNodeSugar for T {
    ///
    /// Updates the child of this tree node
    ///
    fn set_child<TNode: ToTreeNode>(&self, new_node: TNode) {
        self.set_child_ref(new_node.to_tree_node());
    }

    ///
    /// Updates the sibling of this tree node
    ///
    fn set_sibling<TNode: ToTreeNode>(&self, new_node: TNode) {
        self.set_sibling_ref(new_node.to_tree_node());
    }

    ///
    /// Changes the value set for this node. Returns the same node so many nodes can be altered as part of a single statement.
    ///
    fn set_value<TValue: ToTreeValue>(&mut self, new_value: TValue) {
        self.set_tree_value(new_value.to_tree_value());
    }
}
