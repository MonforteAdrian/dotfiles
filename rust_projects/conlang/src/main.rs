mod phonology;
use phonology::{
    MannerOfArticulation, Phoneme, PhonemeInventory, PlaceOfArticulation, VowelBackness,
    VowelHeight,
};

fn main() {
    let inventory = PhonemeInventory::default();

    // Print the default inventory
    println!("Consonants: {:?}", inventory.consonants);
    println!("Vowels: {:?}", inventory.vowels);
}
