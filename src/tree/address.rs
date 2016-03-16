use std::rc::*;

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

    /// Selects a child of this node by looking up the 'nth' child with the specified tag, then selects a new address from there
    ChildWithIndexedTag(String, usize, Box<TreeAddress>)
}

impl TreeNodeIndex for TreeAddress {
    fn lookup_index(&self, parent_node: &Rc<TreeNode>) -> Option<Rc<TreeNode>> {
        match *self {
            TreeAddress::Here => Some(parent_node.to_owned()),
            
            TreeAddress::ChildAtIndex(ref pos, ref next) => {
                (*pos).lookup_index(parent_node).and_then(|new_parent| {
                    (*next).lookup_index(&new_parent)
                })
            },

            TreeAddress::ChildWithTag(ref name, ref next) => {
                (&**name).lookup_index(parent_node).and_then(|new_parent| {
                    (*next).lookup_index(&new_parent)
                })
            },

            TreeAddress::ChildWithIndexedTag(ref name, ref pos, ref next) => {
                let mut current = parent_node.get_child_ref();
                let mut remaining = *pos;

                loop {
                    match current.to_owned() {
                        Some(ref current_ref) => {
                            if current_ref.get_tag() == name {
                                if remaining == 0 {
                                    return (*next).lookup_index(current_ref);
                                }

                                remaining = remaining-1;
                            }

                            current = current_ref.get_sibling_ref();
                        },

                        None => {
                            return None;
                        }
                    }
                }
            }
        }
    }
}

///
/// Structure representing a tag with an index
///
pub struct TagIndex<'a>(&'a str, usize);

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

impl<'a> ToTreeAddress for TagIndex<'a> {
    #[inline]
    fn to_tree_address(&self) -> TreeAddress {
        let TagIndex(ref tag, ref pos) = *self;
        TreeAddress::ChildWithIndexedTag((*tag).to_string(), *pos, Box::new(TreeAddress::Here))
    }

    #[inline]
    fn to_tree_address_then(&self, then: TreeAddress) -> TreeAddress { 
        let TagIndex(ref tag, ref pos) = *self;
        TreeAddress::ChildWithIndexedTag((*tag).to_string(), *pos, Box::new(then))
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
            TreeAddress::Here                                                   => then,
            TreeAddress::ChildAtIndex(ref index, ref old_then)                  => TreeAddress::ChildAtIndex(*index, Box::new((*old_then).to_tree_address_then(then))),
            TreeAddress::ChildWithTag(ref tag, ref old_then)                    => TreeAddress::ChildWithTag((*tag).to_owned(), Box::new((*old_then).to_tree_address_then(then))),
            TreeAddress::ChildWithIndexedTag(ref tag, ref index, ref old_then)  => TreeAddress::ChildWithIndexedTag((*tag).to_owned(), *index, Box::new((*old_then).to_tree_address_then(then)))
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
    fn lookup_index(&self, parent_node: &Rc<TreeNode>) -> Option<Rc<TreeNode>> {
        let Addr(ref first, ref second) = *self;

        first.to_tree_address_then(second.to_tree_address()).lookup_index(parent_node)
    }
}

#[cfg(test)]
mod treeaddress_test {
    #[macro_use]
    use super::super::super::tree::*;

    #[test]
    fn lookup_here() {
        let someTree = tree!("Here", "There", "Everywhere");

        assert!(someTree.get_child_ref_at(TreeAddress::Here).unwrap().get_tag() == "Here");
    }

    #[test]
    fn lookup_child() {
        let someTree = tree!("Here", "There", "Everywhere");

        assert!(someTree.get_child_ref_at(Addr(0, ())).unwrap().get_tag() == "There");
        assert!(someTree.get_child_ref_at(Addr(1, ())).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn lookup_tag() {
        let someTree = tree!("Here", "There", "Everywhere");

        assert!(someTree.get_child_ref_at(Addr("There", ())).unwrap().get_tag() == "There");
        assert!(someTree.get_child_ref_at(Addr("Everywhere", ())).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn lookup_indexed_tag() {
        let someTree = tree!("Here", ("Tag", "First"), ("Tag", "Second"));

        assert!(someTree.get_child_ref_at(Addr(TagIndex("Tag", 0), ())).unwrap().get_value().to_str("") == "First");
        assert!(someTree.get_child_ref_at(Addr(TagIndex("Tag", 1), ())).unwrap().get_value().to_str("") == "Second");
    }

    #[test]
    fn lookup_grandchild() {
        let someTree = tree!("Here", tree!("There", "Everywhere"));

        assert!(someTree.get_child_ref_at(Addr("There", (0, ()))).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn address_after_address() {
        let someTree = tree!("Here", tree!("There", tree!("Everywhere", "Also here")));

        // Address formed of a complicated address with an extra address appended
        let everywhere_address = Addr("There", ("Everywhere", ()));
        assert!(someTree.get_child_ref_at(Addr(everywhere_address, (0, ()))).unwrap().get_tag() == "Also here");
    }
}
