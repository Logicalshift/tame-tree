use super::treenode::*;

///
/// Represents the address of a node relative to another node
///
#[derive(Clone)]
pub enum TreeAddress {
    /// Selects this node
    Here,

    /// Selects a child of this node by index, then selects a new address from there
    ChildAtIndex(usize, Box<TreeAddress>),

    /// Selects a child of this node by tag name, then selects a new address from there
    ChildWithTag(String, Box<TreeAddress>),
}

impl TreeNodeIndex for TreeAddress {
    fn lookup_index(&self, parent_node: &TreeRef) -> Option<TreeRef> {
        match *self {
            TreeAddress::Here => Some(parent_node.to_owned()),
            
            TreeAddress::ChildAtIndex(ref pos, ref next) => {
                pos.lookup_index(parent_node).and_then(|new_parent| {
                    next.lookup_index(&new_parent)
                })
            },

            TreeAddress::ChildWithTag(ref name, ref next) => {
                name.lookup_index(parent_node).and_then(|new_parent| {
                    next.lookup_index(&new_parent)
                })
            }
        }
    }
}

impl PartialEq for TreeAddress {
    fn eq(&self, other: &TreeAddress) -> bool {
        match *self {
            TreeAddress::Here => {
                match *other {
                    TreeAddress::Here   => true,
                    _                   => false
                }
            },

            TreeAddress::ChildAtIndex(self_index, ref self_child) => {
                match *other {
                    TreeAddress::ChildAtIndex(rhs_index, ref rhs_child) => self_index == rhs_index && self_child == rhs_child,
                    _                                                   => false
                }
            },

            TreeAddress::ChildWithTag(ref self_tag, ref self_child) => {
                match *other {
                    TreeAddress::ChildWithTag(ref rhs_tag, ref rhs_child)   => self_tag == rhs_tag && self_child == rhs_child,
                    _                                                       => false
                }
            }
        }
    }
}

impl Eq for TreeAddress {}

impl TreeAddress {
    ///
    /// Returns whether or not address is a parent of this address, or the same address
    ///
    /// Will return None if the two addresses are in incompatible formats (eg, if a tag needs to match up against an indexed address)
    ///
    pub fn is_parent_of(&self, address: &TreeAddress) -> Option<bool> {
        match *self {
            // 'Here' is the root address, the parent of everything (including itself)
            TreeAddress::Here => Some(true),

            // Child addresses must match
            TreeAddress::ChildAtIndex(self_index, ref self_child) => {
                match *address {
                    TreeAddress::ChildAtIndex(address_index, ref address_child) => {
                        if self_index == address_index {
                            self_child.is_parent_of(address_child)
                        } else {
                            Some(false)
                        }
                    },

                    TreeAddress::Here   => Some(false),
                    _                   => None
                }
            },

            TreeAddress::ChildWithTag(ref self_tag, ref self_child) => {
                match *address {
                    TreeAddress::ChildWithTag(ref address_tag, ref address_child) => {
                        if *self_tag == *address_tag {
                            self_child.is_parent_of(address_child)
                        } else {
                            Some(false)
                        }
                    },

                    TreeAddress::Here   => Some(false),
                    _                   => None
                }
            }
        }
    }

    ///
    /// Returns whether or not address is a child of this address or the same address
    ///
    #[inline]
    pub fn is_child_of(&self, address: &TreeAddress) -> Option<bool> {
        address.is_parent_of(self)
    }

    ///
    /// Transforms this address to a new address that is relative to a particular parent address (or None if the addresses 
    /// are in different formats or if parent_address is not a parent of this address)
    ///
    pub fn relative_to(&self, parent_address: &TreeAddress) -> Option<TreeAddress> {
        match *self {
            // Here is a root address, so it doesn't match the parent
            TreeAddress::Here => None,

            // Strip out child addresses
            TreeAddress::ChildAtIndex(self_index, ref self_child) => {
                match *parent_address {
                    // 'Here' is a parent of everything
                    TreeAddress::Here => Some(self.to_owned()),

                    // We carry on down the tree if we get a matching ChildAtIndex address
                    TreeAddress::ChildAtIndex(parent_index, ref parent_child) => {
                        if self_index == parent_index {
                            self_child.relative_to(parent_child)
                        } else {
                            None
                        }
                    },

                    // Other address types count as mismatched (we don't know the tree structure, so we can't match tags against indexes)
                    _ => None
                }
            },

            TreeAddress::ChildWithTag(ref self_tag, ref self_child) => {
                match *parent_address {
                    // 'Here' is a parent of everything
                    TreeAddress::Here => Some(self.to_owned()),

                    // We carry on down the tree if we get a matching ChildWithTag address
                    TreeAddress::ChildWithTag(ref parent_tag, ref parent_child) => {
                        if self_tag == parent_tag {
                            self_child.relative_to(parent_child)
                        } else {
                            None
                        }
                    },

                    // Other address types count as mismatched (we don't know the tree structure, so we can't match tags against indexes)
                    _ => None
                }
            }
        }
    }

    ///
    /// Returns the parent of the current address
    ///
    pub fn parent(&self) -> TreeAddress {
        match *self {
            // 'Here' doesn't have a parent other than itself
            TreeAddress::Here => TreeAddress::Here,

            // The child addresses strip the last child (the one where the address is 'Here')
            TreeAddress::ChildAtIndex(index, ref child) => {
                match **child {
                    TreeAddress::Here   => TreeAddress::Here,
                    _                   => TreeAddress::ChildAtIndex(index, Box::new(child.parent()))
                }
            },

            TreeAddress::ChildWithTag(ref tag, ref child) => {
                match **child {
                    TreeAddress::Here   => TreeAddress::Here,
                    _                   => TreeAddress::ChildWithTag(tag.clone(), Box::new(child.parent()))
                }
            }
        }
    }
}

///
/// Structure representing a shorthand address
///
/// This has `TreeNodeIndex` implemented on it, so `treenode.get_child_ref_at(Addr(0, ()))` will work
///
pub struct Addr<TFirst: ToTreeAddress, TSecond: ToTreeAddress>(TFirst, TSecond);

///
/// Trait that is implemented by types that can be converted to tree addresses
///
pub trait ToTreeAddress {
    fn to_tree_address(&self) -> TreeAddress;
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress;
}

impl ToTreeAddress for () {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        TreeAddress::Here
    }

    #[inline]
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        then
    }
}

impl ToTreeAddress for usize {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        TreeAddress::ChildAtIndex(*self, Box::new(TreeAddress::Here))
    }

    #[inline]
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        TreeAddress::ChildAtIndex(*self, Box::new(then))
    }
}

impl<'a> ToTreeAddress for &'a str {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        TreeAddress::ChildWithTag((*self).to_string(), Box::new(TreeAddress::Here))
    }

    #[inline]
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        TreeAddress::ChildWithTag((*self).to_string(), Box::new(then))
    }
}

impl ToTreeAddress for TreeAddress {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        (*self).to_owned()
    }

    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        match *self {
            TreeAddress::Here                                   => then,
            TreeAddress::ChildAtIndex(ref index, ref old_then)  => TreeAddress::ChildAtIndex(*index, Box::new((*old_then).to_tree_address_then(then))),
            TreeAddress::ChildWithTag(ref tag, ref old_then)    => TreeAddress::ChildWithTag((*tag).to_owned(), Box::new((*old_then).to_tree_address_then(then)))
        }
    }
}

impl<TFirst: ToTreeAddress, TSecond: ToTreeAddress> ToTreeAddress for (TFirst, TSecond) {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        let (ref first, ref second) = *self;

        first.to_tree_address_then(second.to_tree_address())
    }

    #[inline]
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        self.to_tree_address().to_tree_address_then(then)
    }
}

impl<TFirst: ToTreeAddress, TSecond: ToTreeAddress> ToTreeAddress for Addr<TFirst, TSecond> {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        let Addr(ref first, ref second) = *self;

        first.to_tree_address_then(second.to_tree_address())
    }

    #[inline]
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        self.to_tree_address().to_tree_address_then(then)
    }
}

impl<TFirst: ToTreeAddress, TSecond: ToTreeAddress> TreeNodeIndex for Addr<TFirst, TSecond> {
    #[inline]
    fn lookup_index(&self, parent_node: &TreeRef) -> Option<TreeRef> {
        let Addr(ref first, ref second) = *self;

        first.to_tree_address_then(second.to_tree_address()).lookup_index(parent_node)
    }
}

#[cfg(test)]
mod treeaddress_test {
    use super::super::super::tree::*;

    #[test]
    fn lookup_here() {
        let some_tree = tree!("Here", "There", "Everywhere");

        assert!(some_tree.get_child_ref_at(TreeAddress::Here).unwrap().get_tag() == "Here");
    }

    #[test]
    fn lookup_child() {
        let some_tree = tree!("Here", "There", "Everywhere");

        assert!(some_tree.get_child_ref_at(Addr(0, ())).unwrap().get_tag() == "There");
        assert!(some_tree.get_child_ref_at(Addr(1, ())).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn lookup_tag() {
        let some_tree = tree!("Here", "There", "Everywhere");

        assert!(some_tree.get_child_ref_at(Addr("There", ())).unwrap().get_tag() == "There");
        assert!(some_tree.get_child_ref_at(Addr("Everywhere", ())).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn lookup_grandchild() {
        let some_tree = tree!("Here", tree!("There", "Everywhere"));

        assert!(some_tree.get_child_ref_at(Addr("There", (0, ()))).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn address_after_address() {
        let some_tree = tree!("Here", tree!("There", tree!("Everywhere", "Also here")));

        // Address formed of a complicated address with an extra address appended
        let everywhere_address = Addr("There", ("Everywhere", ()));
        assert!(some_tree.get_child_ref_at(Addr(everywhere_address, (0, ()))).unwrap().get_tag() == "Also here");
    }

    #[test]
    fn here_is_parent_of_here() {
        let here        = ().to_tree_address();
        let is_parent   = here.is_parent_of(&here);
        let is_child    = here.is_child_of(&here);

        assert!(is_parent.unwrap());
        assert!(is_child.unwrap());
    }

    #[test]
    fn here_is_parent_of_anything() {
        let here        = ().to_tree_address();
        let there       = (0, (1, 2)).to_tree_address();
        let is_parent   = here.is_parent_of(&there);
        let is_child    = here.is_child_of(&there);

        assert!(is_parent.unwrap());
        assert!(!is_child.unwrap());
    }

    #[test]
    fn nothing_is_parent_of_here() {
        let here        = ().to_tree_address();
        let there       = (0, (1, 2)).to_tree_address();
        let is_parent   = there.is_parent_of(&here);
        let is_child    = there.is_child_of(&here);

        assert!(!is_parent.unwrap());
        assert!(is_child.unwrap());
    }

    #[test]
    fn same_address_is_parent() {
        let here        = (0, (1, 2)).to_tree_address();
        let there       = (0, (1, 2)).to_tree_address();
        let is_parent   = here.is_parent_of(&there);
        let is_child    = here.is_child_of(&there);

        assert!(is_parent.unwrap());
        assert!(is_child.unwrap());
    }

    #[test]
    fn indexed_parent() {
        let here        = (0, 1).to_tree_address();
        let there       = (0, (1, 2)).to_tree_address();
        let is_parent   = here.is_parent_of(&there);
        let is_child    = here.is_child_of(&there);

        assert!(is_parent.unwrap());
        assert!(!is_child.unwrap());
    }

    #[test]
    fn bad_indexed_parent() {
        let here        = (1, 0).to_tree_address();
        let there       = (0, (1, 2)).to_tree_address();
        let is_parent   = here.is_parent_of(&there);
        let is_child    = here.is_child_of(&there);

        assert!(!is_parent.unwrap());
        assert!(!is_child.unwrap());
    }

    #[test]
    fn tagged_parent() {
        let here        = ("first", "second").to_tree_address();
        let there       = ("first", ("second", "third")).to_tree_address();
        let is_parent   = here.is_parent_of(&there);
        let is_child    = here.is_child_of(&there);

        assert!(is_parent.unwrap());
        assert!(!is_child.unwrap());
    }

    #[test]
    fn bad_tagged_parent() {
        let here        = ("other tag", "second").to_tree_address();
        let there       = ("first", ("second", "third")).to_tree_address();
        let is_parent   = here.is_parent_of(&there);
        let is_child    = here.is_child_of(&there);

        assert!(!is_parent.unwrap());
        assert!(!is_child.unwrap());
    }

    #[test]
    fn different_address_types_cant_be_checked() {
        let indexed         = 1.to_tree_address();
        let tagged          = "tag".to_tree_address();

        assert!(indexed.is_parent_of(&tagged).is_none());
        assert!(tagged.is_parent_of(&indexed).is_none());
    }

    #[test]
    fn can_get_relative_address_with_indexes() {
        let address     = (1, (2, (3, 4))).to_tree_address();
        let relative_to = (1, 2).to_tree_address();
        let expected    = (3, 4).to_tree_address();

        assert!(address.relative_to(&relative_to).unwrap() == expected);
    }

    #[test]
    fn can_get_relative_address_with_tags() {
        let address     = ("one", ("two", ("three", "four"))).to_tree_address();
        let relative_to = ("one", "two").to_tree_address();
        let expected    = ("three", "four").to_tree_address();

        assert!(address.relative_to(&relative_to).unwrap() == expected);
    }

    #[test]
    fn relative_to_wrong_address_is_none() {
        let address     = (1, (2, (3, 4))).to_tree_address();
        let relative_to = (3, 4).to_tree_address();

        assert!(address.relative_to(&relative_to).is_none());
    }

    #[test]
    fn relative_to_here_is_none() {
        let address     = ().to_tree_address();
        let relative_to = (3, 4).to_tree_address();

        assert!(address.relative_to(&relative_to).is_none());
    }

    #[test]
    fn get_parent_indexed() {
        let address         = (0, (1, 2)).to_tree_address();
        let parent_address  = address.parent();
        let expected_parent = (0, 1).to_tree_address();

        assert!(parent_address == expected_parent);
    }

    #[test]
    fn get_parent_tagged() {
        let address         = ("tag", ("tag2", "tag3")).to_tree_address();
        let parent_address  = address.parent();
        let expected_parent = ("tag", "tag2").to_tree_address();

        assert!(parent_address == expected_parent);
    }

    #[test]
    fn get_parent_here() {
        assert!(TreeAddress::Here.parent() == TreeAddress::Here);
    }
}
