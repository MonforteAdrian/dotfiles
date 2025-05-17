use crate::{MannerOfArticulation, Phoneme, PlaceOfArticulation, VowelBackness, VowelHeight};

pub struct PhonemeInventory {
    pub consonants: Vec<Phoneme>,
    pub vowels: Vec<Phoneme>,
}

impl PhonemeInventory {
    pub fn new() -> Self {
        Self {
            consonants: Vec::new(),
            vowels: Vec::new(),
        }
    }

    pub fn add_consonant(
        &mut self,
        symbol: char,
        voiced: bool,
        place: PlaceOfArticulation,
        manner: MannerOfArticulation,
    ) {
        self.consonants.push(Phoneme::Consonant {
            symbol,
            voiced,
            place,
            manner,
        });
    }

    pub fn add_vowel(
        &mut self,
        symbol: char,
        rounded: bool,
        height: VowelHeight,
        backness: VowelBackness,
    ) {
        self.vowels.push(Phoneme::Vowel {
            symbol,
            rounded,
            height,
            backness,
        });
    }
}

impl Default for PhonemeInventory {
    fn default() -> Self {
        let mut inventory = PhonemeInventory::new();

        // Add consonants
        inventory.add_consonant(
            'p',
            false,
            PlaceOfArticulation::Bilabial,
            MannerOfArticulation::Plosive,
        );
        inventory.add_consonant(
            'b',
            true,
            PlaceOfArticulation::Bilabial,
            MannerOfArticulation::Plosive,
        );
        inventory.add_consonant(
            't',
            false,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Plosive,
        );
        inventory.add_consonant(
            'd',
            true,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Plosive,
        );
        inventory.add_consonant(
            'k',
            false,
            PlaceOfArticulation::Velar,
            MannerOfArticulation::Plosive,
        );
        inventory.add_consonant(
            'g',
            true,
            PlaceOfArticulation::Velar,
            MannerOfArticulation::Plosive,
        );
        inventory.add_consonant(
            'm',
            true,
            PlaceOfArticulation::Bilabial,
            MannerOfArticulation::Nasal,
        );
        inventory.add_consonant(
            'n',
            true,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Nasal,
        );
        inventory.add_consonant(
            'ɲ',
            true,
            PlaceOfArticulation::Palatal,
            MannerOfArticulation::Nasal,
        );
        inventory.add_consonant(
            'f',
            false,
            PlaceOfArticulation::Labiodenta,
            MannerOfArticulation::Fricative,
        );
        inventory.add_consonant(
            'v',
            true,
            PlaceOfArticulation::Labiodenta,
            MannerOfArticulation::Fricative,
        );
        inventory.add_consonant(
            's',
            false,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Fricative,
        );
        inventory.add_consonant(
            'z',
            true,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Fricative,
        );
        inventory.add_consonant(
            'j',
            true,
            PlaceOfArticulation::Palatal,
            MannerOfArticulation::Approximant,
        );
        inventory.add_consonant(
            'l',
            true,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::LateralApproximant,
        );
        inventory.add_consonant(
            'r',
            true,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Tap,
        );
        inventory.add_consonant(
            'ʙ',
            true,
            PlaceOfArticulation::Bilabial,
            MannerOfArticulation::Trill,
        );
        inventory.add_consonant(
            'r',
            true,
            PlaceOfArticulation::Alveolar,
            MannerOfArticulation::Trill,
        );

        // Add vowels
        inventory.add_vowel('i', false, VowelHeight::Close, VowelBackness::Front);
        inventory.add_vowel('y', true, VowelHeight::Close, VowelBackness::Front);
        inventory.add_vowel('u', true, VowelHeight::Close, VowelBackness::Back);
        inventory.add_vowel('e', false, VowelHeight::Mid, VowelBackness::Front);
        inventory.add_vowel('o', true, VowelHeight::Mid, VowelBackness::Back);
        inventory.add_vowel('a', false, VowelHeight::Open, VowelBackness::Central);

        inventory
    }
}
