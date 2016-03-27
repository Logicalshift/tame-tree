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
