use std::rc::*;

use rustc_serialize::*;

use super::treenode::*;

///
/// Used to help decode tree nodes into other types
///
struct TreeNodeDecoder {
    currentNode: Rc<TreeNode>
}

#[derive(Debug)]
pub enum TreeNodeDecodingError {
    UnsupportedType,
    GenericError(String)
}

impl Decoder for TreeNodeDecoder {
    type Error = TreeNodeDecodingError;

    fn read_nil(&mut self) -> Result<(), Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
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

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_char(&mut self) -> Result<char, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_str(&mut self) -> Result<String, Self::Error> {
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

    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(TreeNodeDecodingError::UnsupportedType)
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
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
