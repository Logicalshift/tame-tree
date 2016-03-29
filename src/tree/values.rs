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

///
/// Represents the possible values of an attribute on a tree node
///
pub enum TreeValue {
    Nothing,
    Bool(bool),
    Int(i32),
    Real(f64),
    String(String),
    Data(Vec<u8>)
}

///
/// Traits implemented by types that can be treated as tree values
///
pub trait ToTreeValue {
    fn to_tree_value(&self) -> TreeValue;
}

impl TreeValue {
    pub fn is_nothing(&self) -> bool {
        match *self {
            TreeValue::Nothing  => true,
            _                   => false
        }
    }

    pub fn to_bool(&self, default: bool) -> bool {
        match *self {
            TreeValue::Bool(ref val)    => *val,
            _                           => default
        }
    }

    pub fn to_int(&self, default: i32) -> i32 {
        match *self {
            TreeValue::Int(ref val)     => *val,
            _                           => default
        }
    }

    pub fn to_real(&self, default: f64) -> f64 {
        match *self {
            TreeValue::Real(ref val)    => *val,
            _                           => default
        }
    }

    pub fn to_str<'a>(&'a self, default: &'a str) -> &'a str {
        match *self {
            TreeValue::String(ref val)  => &**val,
            _                           => default
        }
    }
}

impl ToTreeValue for TreeValue {
    fn to_tree_value(&self) -> TreeValue {
        self.clone()
    }
}

impl<'a> ToTreeValue for &'a TreeValue {
    fn to_tree_value(&self) -> TreeValue {
        (*self).clone()
    }
}

impl ToTreeValue for () {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Nothing }
}

impl ToTreeValue for bool {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Bool(*self) }
}

impl ToTreeValue for i32 {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Int(*self) }
}

impl ToTreeValue for f64 {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Real(*self) }
}

impl<'a> ToTreeValue for &'a str {
    fn to_tree_value(&self) -> TreeValue { TreeValue::String(self.to_string()) }
}

impl ToTreeValue for String {
    fn to_tree_value(&self) -> TreeValue { TreeValue::String(self.to_owned()) }
}

impl ToTreeValue for Vec<u8> {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Data(self.to_owned()) }
}

impl Clone for TreeValue {
    fn clone(&self) -> TreeValue {
        match *self {
            TreeValue::Nothing          => TreeValue::Nothing,
            TreeValue::Bool(b)          => TreeValue::Bool(b),
            TreeValue::Int(v)           => TreeValue::Int(v),
            TreeValue::Real(f)          => TreeValue::Real(f),
            TreeValue::String(ref s)    => TreeValue::String(s.to_string()),
            TreeValue::Data(ref d)      => TreeValue::Data(d.to_vec())
        }
    }
}