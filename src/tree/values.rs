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
