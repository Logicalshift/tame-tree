///
/// Represents the address of a node relative to another node
///
pub enum TreeAddress {
    /// Selects this node
    Here,

    /// Selects a child of this node by index
    ChildIndex(i32, Box<TreeAddress>),

    /// Selects a child of this node by index
    ChildName(String, Box<TreeAddress>)
}
