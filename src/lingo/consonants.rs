/// Trait to extend the `char` type with methods for checking consonants and vowels.
pub trait CharExtensions {
    /// Function to check if the character is a consonant.
    fn is_consonant(self) -> bool;

    /// Function to check if the character is a vowel.
    fn is_vowel(self) -> bool;
}



/// Implementing the `CharExtensions` trait for the `char` type.
/// This adds functionality to `char` to check if it is a consonant or a vowel.
impl CharExtensions for char {
    /// Function to return `true` if the character is a consonant, `false` otherwise.
    fn is_consonant(self) -> bool {
        /* Check if the character is alphabetic and not a vowel. */
        self.is_alphabetic() && !self.is_vowel()
    }

    /// Function to return `true` if the character is a vowel, `false` otherwise.
    fn is_vowel(self) -> bool {
        /* Check if the character is one of the vowels (both lowercase and uppercase). */
        matches!(self, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
}
