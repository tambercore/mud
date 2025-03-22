
/// Type denoting Agda imports.
/// Consists of the package name (String), and the list of components to import.
pub struct Import {pub package : String, pub components : Vec<String>}