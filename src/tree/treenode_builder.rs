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

///
/// Macro that will create a tree from a set of expressions that support the ToTreeNode trait
///
/// The first parameter is the root item, followed by the child items
///
#[macro_export]
macro_rules! tree {
    ( $root: expr, $( $child: expr ), * ) => {
        {
            let root            = $root.to_tree_node();
            let mut child_list  = Vec::new();

            $(
                child_list.push($child.to_tree_node());
            )*

            root.with_children(&child_list)
        }
    }
}

#[cfg(test)]
mod treenode_builder_tests {
    use super::super::treenode::*;

    #[test]
    fn can_build_tree_macro() {
        let root = tree!("root", "child1", ("child2", "value"), tree!("child3", "grandchild1"));

        assert!(root.get_child_ref().is_some());
        assert!(root.get_child_ref_at(0).map(|x| x.get_tag() == "child1").unwrap_or(false));
        assert!(root.get_child_ref_at(1).map(|x| x.get_tag() == "child2").unwrap_or(false));
        assert!(root.get_child_ref_at(2).map(|x| x.get_tag() == "child3").unwrap_or(false));
        assert!(root.get_child_ref_at(2).and_then(|x| x.get_child_ref_at(0)).map(|x| x.get_tag() == "grandchild1").unwrap_or(false));
        assert!(root.get_child_ref_at(3).is_none());
    }
}
