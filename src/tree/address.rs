use std::rc::*;

use super::treenode::*;

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

        assert!(someTree.get_child_ref_at(TreeAddress::ChildAtIndex(0, Box::new(TreeAddress::Here))).unwrap().get_tag() == "There");
        assert!(someTree.get_child_ref_at(TreeAddress::ChildAtIndex(1, Box::new(TreeAddress::Here))).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn lookup_tag() {
        let someTree = tree!("Here", "There", "Everywhere");

        assert!(someTree.get_child_ref_at(TreeAddress::ChildWithTag("There".to_string(), Box::new(TreeAddress::Here))).unwrap().get_tag() == "There");
        assert!(someTree.get_child_ref_at(TreeAddress::ChildWithTag("Everywhere".to_string(), Box::new(TreeAddress::Here))).unwrap().get_tag() == "Everywhere");
    }

    #[test]
    fn lookup_indexed_tag() {
        let someTree = tree!("Here", ("Tag", "First"), ("Tag", "Second"));

        assert!(someTree.get_child_ref_at(TreeAddress::ChildWithIndexedTag("Tag".to_string(), 0, Box::new(TreeAddress::Here))).unwrap().get_value().to_str("") == "First");
        assert!(someTree.get_child_ref_at(TreeAddress::ChildWithIndexedTag("Tag".to_string(), 1, Box::new(TreeAddress::Here))).unwrap().get_value().to_str("") == "Second");
    }

    #[test]
    fn lookup_grandchild() {
        let someTree = tree!("Here", tree!("There", "Everywhere"));

        assert!(someTree.get_child_ref_at(TreeAddress::ChildWithTag("There".to_string(), Box::new(TreeAddress::ChildAtIndex(0, Box::new(TreeAddress::Here))))).unwrap().get_tag() == "Everywhere");
    }
}
