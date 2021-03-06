//
//   Copyright 2016 Andrew Hunter
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
//

use std::result::*;
use std::rc::*;

use rustc_serialize::*;

use super::treenode::*;
use super::basictree::*;
use super::values::*;

///
/// Encoder that will write to the specified tree node 
///
struct TreeNodeEncoder {
    tag:    String,
    value:  TreeValue,
    child:  Option<TreeRef>
}

impl TreeNodeEncoder {
    fn new() -> TreeNodeEncoder {
        TreeNodeEncoder { 
            tag:    "".to_string(), 
            value:  TreeValue::Nothing,
            child:  None }
    }

    fn to_basic_tree_node_with_sibling(&self, new_sibling: Option<TreeRef>) -> BasicTree {
        let new_node = BasicTree::new(&*self.tag, self.value.to_owned(), self.child.to_owned(), new_sibling);

        new_node
    }
}

#[derive(Debug)]
pub enum TreeNodeCodingError {
    UnsupportedType
}

#[allow(unused_variables)]          // Unused function parameters are quite common due to the way this trait is designed
impl Encoder for TreeNodeEncoder {
    type Error = TreeNodeCodingError;

    fn emit_nil(&mut self) -> Result<(), Self::Error> {
        self.value = TreeValue::Nothing;
        Ok(())
    }

    fn emit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.value = TreeValue::Int(v);
        Ok(())
    }

    fn emit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.value = TreeValue::Int(v as i32);
        Ok(())
    }

    fn emit_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.value = TreeValue::Int(v as i32);
        Ok(())
    }

    fn emit_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.value = TreeValue::Bool(v);
        Ok(())
    }

    fn emit_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        self.value = TreeValue::Real(v);
        Ok(())
    }

    fn emit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.value = TreeValue::Real(v as f64);
        Ok(())
    }

    fn emit_str(&mut self, v: &str) -> Result<(), Self::Error> {
        self.value = TreeValue::String(v.to_string());
        Ok(())
    }

    fn emit_struct<F>(&mut self, name: &str, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        self.value = TreeValue::String(name.to_string());

        f(self)
    }

    fn emit_struct_field<F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        // Encode the function into a new encoder
        let mut node_encoder = TreeNodeEncoder::new();
        let encoding_result = f(&mut node_encoder);

        node_encoder.tag = f_name.to_string();

        // Short-circuit on error
        if encoding_result.is_err() {
            return encoding_result;
        }

        // Replace the child node with the node generated for the new encoder
        let new_node = node_encoder.to_basic_tree_node_with_sibling(self.child.to_owned());

        // Save the node we just created and update the tree
        self.child = Some(Rc::new(new_node));

        Ok(())
    }

    fn emit_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_char(&mut self, v: char) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_enum<F>(&mut self, name: &str, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_enum_variant<F>(&mut self, v_name: &str, v_id: usize, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_enum_variant_arg<F>(&mut self, a_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_enum_struct_variant<F>(&mut self, v_name: &str, v_id: usize, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_enum_struct_variant_field<F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_tuple<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_tuple_arg<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_tuple_struct<F>(&mut self, name: &str, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_tuple_struct_arg<F>(&mut self, f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_option<F>(&mut self, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_option_none(&mut self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_option_some<F>(&mut self, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_seq_elt<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_map<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_map_elt_key<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }

    fn emit_map_elt_val<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(TreeNodeCodingError::UnsupportedType)
    }
}

///
/// Converts an encodable object into a treenode
///
pub fn encode<T: Encodable>(source: &T) -> Result<Rc<TreeNode>, TreeNodeCodingError> {
    // The encoder doesn't directly create a TreeNode because of the way rust lifetimes work
    // (We'd need a <'a> lifetime on the encoder, and that lifetime would prevent recursion by generating
    // new encoders. This is really a limitation of Rust; we work around it by generating the description of
    // a tree node in the encoder and then the tree node itself outside of it)
    //
    // We don't expose the actual encoder publically for this reason, the API is too dumb by necessity.
    let mut encoder = TreeNodeEncoder::new();
    let result = source.encode(&mut encoder);

    result.map(|_| {
        let result: Rc<TreeNode> = Rc::new(encoder.to_basic_tree_node_with_sibling(None));
        result
    })
}

///
/// Marker trait that can be added to types to make them support encoding to a tree node via .to_tree_node()
///
pub trait EncodeToTreeNode {
    // Empty, this is a marker type
}

impl EncodeToTreeNode for String {}
impl EncodeToTreeNode for i32 {}
impl EncodeToTreeNode for f64 {}
impl EncodeToTreeNode for Vec<u8> {}

impl<T: Encodable + EncodeToTreeNode> ToTreeNode for T {
    ///
    /// Converts this value into a tree node
    ///
    fn to_tree_node(&self) -> Rc<TreeNode> {
        encode(self).unwrap()
    }
}

#[cfg(test)]
mod serialize_tests {
    use super::super::super::tree::*;

    #[derive(RustcEncodable, RustcDecodable)]
    struct Test {
        field1: i32,
        field2: String,
        field3: bool
    }

    // One day this ought to be possible via #[derive]
    impl EncodeToTreeNode for Test { }

    #[test]
    fn encode_struct() {
        let test = Test { field1: 32, field2: "Hi".to_string(), field3: true };
        let encoded = test.to_tree_node();

        assert!(match *encoded.get_child_at("field1").get_value() { TreeValue::Int(ref x) => *x == 32, _ => false });
        assert!(match *encoded.get_child_at("field2").get_value() { TreeValue::String(ref x) => *x == "Hi", _ => false });
        assert!(match *encoded.get_child_at("field3").get_value() { TreeValue::Bool(ref x) => *x == true, _ => false });
    }
}
