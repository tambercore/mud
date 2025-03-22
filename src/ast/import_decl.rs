
/// Type denoting Agda imports.
/// Consists of the package name (String), and the list of components to import.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Import {pub package : String, pub components : Vec<String>}