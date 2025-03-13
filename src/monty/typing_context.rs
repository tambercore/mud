use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Type alias for a typing context that stores identifier-type pairs.
pub(crate) type TypingContext = HashMap<String, String>;

/// Static variable holding a `Mutex`-protected `TypingContext`.
/// This provides safe concurrent access to the typing context.
pub static TYPING_CONTEXT: Lazy<Mutex<TypingContext>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Function to reset the typing context by clearing its contents.
pub fn reset_typing_context() {
    // Lock the context and replace it with an empty HashMap.
    let mut context = TYPING_CONTEXT.lock().unwrap();
    *context = HashMap::new(); // Replace with an empty HashMap
}

/// Function to insert an identifier and its associated type into the typing context.
pub fn insert_into_context(iden: String, typ: String) {
    /* Lock the context and insert the identifier-type pair.*/
    let mut context = TYPING_CONTEXT.lock().unwrap();
    context.insert(iden, typ);
}
