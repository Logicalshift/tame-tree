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

use rustc_serialize::*;

use super::encoder::*;
use super::treenode::*;
use super::values::*;

///
/// Used to help decode tree nodes into other types
///
struct TreeNodeDecoder {
    current_node: TreeRef
}

#[derive(Debug)]
pub enum TreeNodeDecodingError {
    UnsupportedType,
    NodeHasInvalidType,
    ValueOutOfRange,
    MissingField,
    GenericError(String)
}

impl TreeNodeDecoder {
    fn read_current(&self) -> &TreeValue {
        self.current_node.get_value()
    }
}

#[allow(unused_variables)]          // Unused function parameters are quite common due to the way this trait is designed
impl Decoder for TreeNodeDecoder {
    type Error = TreeNodeDecodingError;

    fn read_nil(&mut self) -> Result<(), Self::Error> {
        match *self.read_current() {
            TreeValue::Nothing  => Ok(()),
            _                   => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        match *self.read_current() {
            TreeValue::Int(ref x)   => Ok(*x),
            _                       => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        match *self.read_current() {
            TreeValue::Int(ref x)   => if (*x >= i16::min_value() as i32) && (*x <= i16::max_value() as i32) { Ok(*x as i16) } else { Err(TreeNodeDecodingError::ValueOutOfRange) },
            _                       => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        match *self.read_current() {
            TreeValue::Int(ref x)   => if (*x >= i8::min_value() as i32) && (*x <= i8::max_value() as i32) { Ok(*x as i8) } else { Err(TreeNodeDecodingError::ValueOutOfRange) },
            _                       => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_str(&mut self) -> Result<String, Self::Error> {
        match *self.read_current() {
            TreeValue::String(ref x)    => Ok(x.to_owned()),
            _                           => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        match *self.read_current() {
            TreeValue::Bool(ref x)  => Ok(*x),
            _                       => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        match *self.read_current() {
            TreeValue::Real(ref x)  => Ok(*x),
            _                       => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        match *self.read_current() {
            TreeValue::Real(ref x)  => Ok(*x as f32),
            _                       => Err(TreeNodeDecodingError::NodeHasInvalidType)
        }
    }

    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        // Look up the field
        // TODO: could hash the field names to avoid doing a linear search every time (not clear if there are substantial benefits for this given the small number of fields in most structures)
        let field = self.current_node.get_child_ref_at(f_name);

        match field {
            None        => Err(TreeNodeDecodingError::MissingField),
            Some(ref x) => {
                // Move into the field node
                let previous_node = self.current_node.to_owned();
                self.current_node = x.to_owned();

                // Decode it
                let result = f(self);

                // Move back out
                self.current_node = previous_node.to_owned();

                result
            }
        }
    }

    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_isize(&mut self) -> Result<isize, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_char(&mut self) -> Result<char, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_enum<T, F>(&mut self, name: &str, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_tuple<T, F>(&mut self, len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_option<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, bool) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_map<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn error(&mut self, err: &str) -> Self::Error {
        TreeNodeDecodingError::GenericError(err.to_string())
    }
}

///
/// Trait implemented by things that can be decoded from a tree node
///
pub trait DecodeFromTreeNode : Sized {
    ///
    /// Creates a new object from a tree node
    ///
    fn new_from_tree(tree: &TreeRef) -> Result<Self, TreeNodeDecodingError>;
}

impl<T: Decodable + EncodeToTreeNode> DecodeFromTreeNode for T {
    ///
    /// Creates a new object from a tree node
    ///
    fn new_from_tree(tree: &TreeRef) -> Result<T, TreeNodeDecodingError> {
        let mut decoder = TreeNodeDecoder { current_node: tree.to_owned() };

        T::decode(&mut decoder)
    }
}

#[cfg(test)]
mod decoder_tests {
    use super::super::super::tree::*;

    #[derive(RustcEncodable, RustcDecodable)]
    struct Test {
        field1: i32,
        field2: String,
        field3: bool
    }

    impl EncodeToTreeNode for Test { }

    #[test]
    fn encode_decode_structure() {
        let initial_structure = Test { field1: 42, field2: "test string".to_string(), field3: true };

        let encoded = initial_structure.to_tree_node();
        let decoded = Test::new_from_tree(&encoded);

        assert!(decoded.is_ok());

        let result = decoded.unwrap();

        assert!(result.field1 == 42);
        assert!(result.field2 == "test string");
        assert!(result.field3);
    }
}
