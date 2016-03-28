use std::rc::*;
use std::iter::*;

use super::treenode::*;
use super::extent::*;

///
/// Iterates over a tree node
///
pub trait TreeIterator {
    /// Returns the next item in this tree
    fn next_in_tree(&mut self) -> Option<TreeRef>;
}

pub trait TreeNodeIteration {
    ///
    /// Creates an iterator for a particular extent of the tree
    ///
    fn iter_extent(&self, extent: TreeExtent) -> Box<TreeIterator>;

    ///
    /// Creates an iterator that covers the child nodes of this node
    ///
    fn iter_children(&self) -> Box<TreeIterator>;
}

impl Iterator for Box<TreeIterator> {
    type Item = TreeRef;

    #[inline]
    fn next(&mut self) -> Option<TreeRef> {
        self.next_in_tree()
    }
}

impl TreeNodeIteration for TreeRef {
    ///
    /// Creates an iterator for a particular extent of the tree
    ///
    fn iter_extent(&self, extent: TreeExtent) -> Box<TreeIterator> {
        match extent {
            TreeExtent::ThisNode => Box::new(HereIterator::new(self.to_owned())),
            TreeExtent::Children => self.iter_children(),

            TreeExtent::SubTree => {
                // Don't perform a search of the siblings of this item (combine the 'here' and the 'depth first' iterators)
                let here        = Box::new(HereIterator::new(self.to_owned()));
                let child_opt   = self.get_child_ref();

                match child_opt {
                    Some(child) => Box::new(ChainedIterator::new(here, Box::new(DepthSearchIterator::new(child)))),
                    None        => here
                }
            }
        }
    }

    ///
    /// Creates an iterator that covers the child nodes of this node
    ///
    fn iter_children(&self) -> Box<TreeIterator> {
        let child_opt = self.get_child_ref();

        match child_opt {
            Some(child) => Box::new(SiblingIterator::new(child)),
            None        => Box::new(NoIterator::new())
        }
    }
}

///
/// Iterator representing no tree nodes
///
struct NoIterator;

impl NoIterator {
    fn new() -> NoIterator {
        NoIterator
    }
}

impl TreeIterator for NoIterator {
    fn next_in_tree(&mut self) -> Option<TreeRef> {
        None
    }
}

///
/// Iterates through the siblings of a particular tree node
///
struct SiblingIterator {
    current: Option<TreeRef>
}

impl SiblingIterator {
    fn new(start: TreeRef) -> SiblingIterator {
        SiblingIterator { current: Some(start) }
    }
}

impl TreeIterator for SiblingIterator {
    fn next_in_tree(&mut self) -> Option<TreeRef> {
        let result = self.current.to_owned();

        let next = match self.current {
            Some(ref node)  => node.get_sibling_ref(),
            None            => None
        };

        self.current = next;
        result
    }
}

///
/// Iterates across a single tree node
///
struct HereIterator {
    current: Option<TreeRef>
}

impl HereIterator {
    #[inline]
    fn new(here: TreeRef) -> HereIterator {
        HereIterator { current: Some(here) }
    }
}

impl TreeIterator for HereIterator {
    fn next_in_tree(&mut self) -> Option<TreeRef> {
        let result = self.current.to_owned();

        self.current = None;
        result
    }
}

///
/// Iterates across a whole tree using a depth-first search
///
struct DepthSearchIterator {
    stack: Vec<TreeRef>
}

impl DepthSearchIterator {
    #[inline]
    fn new(start: TreeRef) -> DepthSearchIterator {
        DepthSearchIterator { stack: vec!(start) }
    }
}

impl TreeIterator for DepthSearchIterator {
    fn next_in_tree(&mut self) -> Option<TreeRef> {
        // Pop from the stack
        let current = self.stack.pop();

        let result = match current {
            Some(ref node) => {
                // Iterate the children then the siblings of this node
                let child   = node.get_child_ref();
                let sibling = node.get_sibling_ref();

                match sibling {
                    Some(s) => self.stack.push(s),
                    None    => {}
                }

                match child {
                    Some(c) => self.stack.push(c),
                    None    => {}
                };

                // Result is the current node
                Some(node.to_owned())
            },
            None => None
        };

        result
    }
}

///
/// Chains two tree iterators
///
struct ChainedIterator {
    iterators: Vec<Box<TreeIterator>>
}

impl ChainedIterator {
    fn new(first: Box<TreeIterator>, second: Box<TreeIterator>) -> ChainedIterator {
        ChainedIterator { iterators: vec!(second, first) }
    }
}

impl TreeIterator for ChainedIterator {
    fn next_in_tree(&mut self) -> Option<TreeRef> {
        let result = {
            let active = self.iterators.last_mut();

            match active {
                Some(iterator) => {
                    let active_result = iterator.next_in_tree();
                    active_result
                },

                None => None
            }
        };

        if result.is_none() && !self.iterators.is_empty() {
            self.iterators.pop();
            self.next_in_tree()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::super::super::tree::*;

    #[test]
    fn iterate_children() {
        let tree        = tree!(("root", 0), ("", 1), ("", 2), ("", 3), tree!(("", 4), ("grandchild", 5)));
        let iterator    = tree.iter_children().map(|x| x.get_value().to_int(-1));
        let collected   = iterator.collect::<Vec<i32>>();

        assert!(collected == vec!(1, 2, 3, 4));
    }

    #[test]
    fn iterate_no_children() {
        let tree        = tree!(("root", 0), ("", 1), ("", 2), ("", 3), tree!(("", 4), ("grandchild", 5)));
        let iterator    = tree.get_child_ref().unwrap().iter_children().map(|x| x.get_value().to_int(-1));
        let collected   = iterator.collect::<Vec<i32>>();

        assert!(collected == vec!());
    }

    #[test]
    fn iterate_children_extent() {
        let tree        = tree!(("root", 0), ("", 1), ("", 2), ("", 3), tree!(("", 4), ("grandchild", 5)));
        let iterator    = tree.iter_extent(TreeExtent::Children).map(|x| x.get_value().to_int(-1));
        let collected   = iterator.collect::<Vec<i32>>();

        assert!(collected == vec!(1, 2, 3, 4));
    }

    #[test]
    fn iterate_this_node() {
        let tree        = tree!(("root", 0), ("", 1), ("", 2), ("", 3), tree!(("", 4), ("grandchild", 5)));
        let iterator    = tree.iter_extent(TreeExtent::ThisNode).map(|x| x.get_value().to_int(-1));
        let collected   = iterator.collect::<Vec<i32>>();

        assert!(collected == vec!(0));
    }

    #[test]
    fn iterate_subtree() {
        let tree        = tree!(("root", 0), ("", 1), ("", 2), tree!(("", 3), ("", 4)), ("", 5));
        let iterator    = tree.iter_extent(TreeExtent::SubTree).map(|x| x.get_value().to_int(-1));
        let collected   = iterator.collect::<Vec<i32>>();

        assert!(collected == vec!(0, 1, 2, 3, 4, 5));
    }

    #[test]
    fn iterate_subtree_without_siblings() {
        let tree        = tree!(("root", 0), ("", 1), ("", 2), tree!(("", 3), ("", 4)), ("", 5));
        let iterator    = tree.get_child_ref().unwrap().iter_extent(TreeExtent::SubTree).map(|x| x.get_value().to_int(-1));
        let collected   = iterator.collect::<Vec<i32>>();

        assert!(collected == vec!(1));
    }
}
