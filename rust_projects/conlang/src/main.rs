const CONSONANTS: [char; 6] = ['p', 'b', 't', 'd', 'k', 'g'];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn diphthongs() {
    for i in VOWELS {
        for j in VOWELS {
            println!("{i}{j}");
        }
    }
}

fn main() {
    diphthongs();
}
