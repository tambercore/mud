use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::top_decl::TDeclaration;


/// A thread-safe static storage for Agda expression interpretations.
/// This is a `Lazy`-initialized `Mutex<HashMap<TDeclaration, String>>`
/// which holds the interpretations for Agda expressions.
pub static INTERPRETATIONS: Lazy<Mutex<HashMap<TDeclaration, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


/// Retrieves the interpretation for a given Agda expression (TDeclaration).
pub fn get_interpretation(agda_expr: &TDeclaration) -> Option<String> {
    let map = INTERPRETATIONS.lock().unwrap();
    map.get(agda_expr).cloned()
}



/// Inserts a new interpretation for a given Agda expression (TDeclaration).
pub fn insert_interpretation(agda_expr: TDeclaration, interpretation: String) {
    // Lock the Mutex for safe, mutable access
    let mut map = INTERPRETATIONS.lock().unwrap();

    // Insert the new interpretation into the HashMap
    map.insert(agda_expr, interpretation);
}


/// Prints all stored interpretations in the `INTERPRETATIONS` map.
/// This will iterate over all stored `TDeclaration` to `String` mappings and print them.
pub fn print_interpretations() {
    // Lock the Mutex for safe, mutable access
    let map = INTERPRETATIONS.lock().unwrap();

    for (key, value) in map.iter() {
        println!("{:?} : {}\n\n", key, value);
    }
}
