use std::sync::atomic::{AtomicUsize, Ordering};

/// Global atomic counter for generating fresh variable names.
static COUNTER: AtomicUsize = AtomicUsize::new(0);



/// Converts a number into its Unicode subscript representation.
/// For example, 12 becomes "₁₂".
pub fn to_unicode_subscript(n: usize) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '0' => '₀',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            _ => c,
        })
        .collect()
}



/// Function to return a fresh variable name like "x₁", "x₂", "x₃", etc.
pub fn fresh_variable() -> String {
    let count = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("x{}", to_unicode_subscript(count))
}



/// Function to reset the global counter to zero.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::Relaxed);
}