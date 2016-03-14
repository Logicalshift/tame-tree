///
/// Represents the address of a node relative to another node
///
pub enum TreeAddress {
    /// Selects this node
    Here,

    /// Selects a child of this node by index, then selects a new address from there
    ChildAtIndex(usize, Box<TreeAddress>),

    /// Selects a child of this node by tag name, then selects a new address from there
    ChildWithTag(String, Box<TreeAddress>),

    /// Selects a child of this node by looking up the 'nth' child with the specified tag, then selects a new address from there
    ChildWithIndexedTag(String, usize, Box<TreeAddress>)
}
