use crate::PhonemeInventory;

pub struct PhonotacticRule {
    pub pattern: String, // e.g., "CVC"
}

impl PhonotacticRule {
    pub fn is_valid(&self, word: &str, inventory: &PhonemeInventory) -> bool {
        let mut pattern_chars = self.pattern.chars();
        for c in word.chars() {
            match pattern_chars.next() {
                Some('C') => {
                    if !inventory.consonants.iter().any(|p| p.symbol() == c) {
                        return false;
                    }
                }
                Some('V') => {
                    if !inventory.vowels.iter().any(|p| p.symbol() == c) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        pattern_chars.next().is_none()
    }
}
