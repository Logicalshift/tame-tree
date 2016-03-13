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
    child:  Option<Rc<TreeNode>>
}

impl TreeNodeEncoder {
    fn new() -> TreeNodeEncoder {
        TreeNodeEncoder { 
            tag:    "".to_string(), 
            value:  TreeValue::Nothing,
            child:  None }
    }
}

pub enum TreeNodeCodingError {
    UnsupportedType
}

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
        // Hrm, so what I want to do is create a new encoder with a new node and call f on it.
        // But rust has other ideas; it doesn't know that f(X) doesn't reference X after it returns, so it moans
        // Other ideas
        //   * create a whole new encoder (can't do it, we don't have any access to the struct)
        //   * swap the reference to the node (can't do it, the new node has the same lifetime problems)
        //   * use a CloneCell (can't do it, set_tree_value and set_tag aren't supported)

        // Insert a new node into the tree
        let new_node = BasicTree::new(f_name, ());

        self.child.to_owned().and_then(|sibling| {
            new_node.set_sibling_ref(sibling);
            Some(())
        });

        // Encode the field into the new node (would be super-elegant if this works but Rust is all 'nooope, you need to write a 
        // billion more lines of code'). I know *why* but this is stupid, if the encoder function had a better lifetime specifier
        // it'd be unnecessary.
        // Rust thinks that f() needs stuff with a lifetime the same as this struct rather than stuff with a lifetime as long
        // as the function call (it's not obvious from a casual reading of the definition, rust likes to be inscrutable). 
        // This means we need to do dumb stuff to make it work.
        let mut node_encoder = TreeNodeEncoder::new();
        f(&mut node_encoder);

        // Save the node we just created and update the tree
        self.child = Some(Rc::new(new_node));

        Err(TreeNodeCodingError::UnsupportedType)
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
