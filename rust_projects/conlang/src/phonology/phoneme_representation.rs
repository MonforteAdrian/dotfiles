#[derive(Debug, Clone)]
pub enum PlaceOfArticulation {
    Bilabial,
    Labiodenta,
    Dental,
    Alveolar,
    PostAlveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Phayngeal,
    Glottal,
}

#[derive(Debug, Clone)]
pub enum MannerOfArticulation {
    Plosive,
    Nasal,
    Trill,
    Tap,
    Fricative,
    LateralFricative,
    Approximant,
    LateralApproximant,
}

#[derive(Debug, Clone)]
pub enum VowelHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

#[derive(Debug, Clone)]
pub enum VowelBackness {
    Front,
    Central,
    Back,
}

#[derive(Debug, Clone)]
pub enum Phoneme {
    Consonant {
        symbol: char,
        voiced: bool,
        place: PlaceOfArticulation,
        manner: MannerOfArticulation,
    },
    Vowel {
        symbol: char,
        rounded: bool,
        height: VowelHeight,
        backness: VowelBackness,
    },
}

impl Phoneme {
    pub fn symbol(&self) -> char {
        match self {
            Phoneme::Consonant { symbol, .. } => *symbol,
            Phoneme::Vowel { symbol, .. } => *symbol,
        }
    }
}
