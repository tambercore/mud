
pub trait CharExtensions {
    fn is_consonant(self) -> bool;
    fn is_vowel(self) -> bool;
}

impl CharExtensions for char {
    fn is_consonant(self) -> bool {
        self.is_alphabetic() && !self.is_vowel()
    }

    fn is_vowel(self) -> bool {
        matches!(self, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
}
