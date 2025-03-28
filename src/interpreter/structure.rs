use std::collections::HashMap;
use std::sync::Mutex;
use crate::ast::agda_expr::AgdaExpr;
use crate::ast::top_decl::TDeclaration;

lazy_static::lazy_static! {
    pub static ref INTERPRETATIONS: Mutex<HashMap<TDeclaration, String>> = Mutex::new(HashMap::new());
}

pub fn insert_interpretation(agda_expr: TDeclaration, interpretation: String) {
    // Lock the Mutex for safe, mutable access
    let mut map = INTERPRETATIONS.lock().unwrap();

    // Insert the new interpretation into the HashMap
    map.insert(agda_expr, interpretation);
}

pub fn print_interpretations() {
    // Lock the Mutex for safe, mutable access
    let map = INTERPRETATIONS.lock().unwrap();

    for (key, value) in map.iter() {
        println!("{:?} : {}\n\n", key, value);
    }
}
