use super::address::*;

///
/// An extent represents a series of nodes starting at a specified node
///
pub enum TreeExtent {
    /// Just the initial node
    ThisNode,

    /// The children of this node
    ///
    /// This does not extend beyond the immediate children of the current node.
    Children,

    /// The entire subtree (all children, and their children, and so on)
    ///
    /// Unlike Children, this covers the current node and its entire subtree
    SubTree
}

impl TreeExtent {
    ///
    /// Returns true if this extent will cover the specified address, which is relative to where the extent starts
    ///
    pub fn covers(&self, address: &TreeAddress) -> bool {
        match *self {
            TreeExtent::ThisNode => {
                match *address {
                    TreeAddress::Here   => true,
                    _                   => false
                }
            },

            TreeExtent::Children => {
                match *address {
                    TreeAddress::ChildAtIndex(_, ref child_address) => TreeExtent::ThisNode.covers(child_address),
                    TreeAddress::ChildWithTag(_, ref child_address) => TreeExtent::ThisNode.covers(child_address),
                    _                                               => false
                }
            },

            TreeExtent::SubTree => true
        }
    }
}

#[cfg(test)]
mod extent_tests {
    use super::super::super::tree::*;

    #[test]
    fn thisnode_covers_only_here() {
        assert!(TreeExtent::ThisNode.covers(&TreeAddress::Here));
        assert!(!TreeExtent::ThisNode.covers(&(1.to_tree_address())));
    }

    #[test]
    fn children_covers_only_immediate_children() {
        assert!(TreeExtent::Children.covers(&(1.to_tree_address())));
        assert!(TreeExtent::Children.covers(&("tag".to_tree_address())));

        assert!(!TreeExtent::Children.covers(&((1, 2).to_tree_address())));
        assert!(!TreeExtent::Children.covers(&(("tag", "othertag").to_tree_address())));
        assert!(!TreeExtent::Children.covers(&TreeAddress::Here));
    }

    #[test]
    fn subtree_covers_everything() {
        assert!(TreeExtent::SubTree.covers(&(1.to_tree_address())));
        assert!(TreeExtent::SubTree.covers(&("tag".to_tree_address())));

        assert!(TreeExtent::SubTree.covers(&((1, 2).to_tree_address())));
        assert!(TreeExtent::SubTree.covers(&(("tag", "othertag").to_tree_address())));
        assert!(TreeExtent::SubTree.covers(&TreeAddress::Here));
    }
}
