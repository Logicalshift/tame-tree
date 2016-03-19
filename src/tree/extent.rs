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
    SubTree,

    /// Nodes beginning at the initial node and then iterated over using a depth-first algorithm for the specified number of iterations
    ///
    /// May include siblings of the initial node. This type of extent has the property that it's not possible to tell if a particular
    /// relative address is within the extent without also knowing the structure of the tree.
    ///
    /// This extent is useful for communicating information about tree differences in the form of a stream than as a tree, as the
    /// 'depth first' enumeration is a good way to serialize a tree.
    DepthFirst(i32)
}
