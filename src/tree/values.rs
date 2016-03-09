///
/// Represents the possible values of an attribute on a tree node
///
pub enum TreeValue {
    Nothing,
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

impl ToTreeValue for TreeValue {
    fn to_tree_value(&self) -> TreeValue { 
        match *self {
            TreeValue::Nothing          => TreeValue::Nothing,
            TreeValue::Int(v)           => TreeValue::Int(v),
            TreeValue::Real(f)          => TreeValue::Real(f),
            TreeValue::String(ref s)    => TreeValue::String(s.to_string()),
            TreeValue::Data(ref d)      => TreeValue::Data(d.to_vec())
        }
    }
}

impl ToTreeValue for () {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Nothing }
}

impl ToTreeValue for i32 {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Int(*self) }
}

impl ToTreeValue for f64 {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Real(*self) }
}

impl ToTreeValue for str {
    fn to_tree_value(&self) -> TreeValue { TreeValue::String(self.to_owned()) }
}

impl ToTreeValue for String {
    fn to_tree_value(&self) -> TreeValue { TreeValue::String(self.to_owned()) }
}

impl ToTreeValue for Vec<u8> {
    fn to_tree_value(&self) -> TreeValue { TreeValue::Data(self.to_owned()) }
}
