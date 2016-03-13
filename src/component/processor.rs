use std::rc::*;

use super::super::tree::*;

///
/// A processor is a type that converts a tree into another tree
///
/// This is the simplest type of component. It doesn't react to changes to the input tree using any
/// kind of persistent state, and it's not capable of dealing with partial changes.
///
pub trait Processor {
    ///
    /// Transforms an input tree into an output tree
    ///
    fn process(&self, input_tree: Rc<TreeNode>) -> Rc<TreeNode>;
}
