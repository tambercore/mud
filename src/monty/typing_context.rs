use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub(crate) type TypingContext = HashMap<String, String>;

pub static TYPING_CONTEXT: Lazy<Mutex<TypingContext>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn reset_typing_context() {
    let mut context = TYPING_CONTEXT.lock().unwrap();
    *context = HashMap::new(); // Replace with an empty HashMap
}

pub fn insert_into_context(iden: String, typ: String) {
    let mut context = TYPING_CONTEXT.lock().unwrap();
    context.insert(iden, typ);

}