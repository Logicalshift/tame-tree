///
/// Represents the possible values of an attribute on a tree node
///
pub enum AttributeValue {
    Int(i32),
    Real(f64),
    String(String),
    Data(Vec<u8>)
}

///
/// Attributes are used to attach values to tree nodes.
///
pub struct Attribute {
    name: String,
    value: AttributeValue
}

impl Attribute {
    pub fn get_name(&self) -> &String { return &self.name; }
    pub fn get_value(&self) -> &AttributeValue { return &self.value; }
}
