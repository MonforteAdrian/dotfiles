const CONSONANTS_ONSET: [&str; 21] = ["p", "b", "t", "d", "k", "g", "m", "n", "ɲ", "p͡f", "t͡s", "f", "v", "s", "z", "x", "j", "r", "ʙ", "R", "l"];
const CONSONANTS_PAIRS_ONSET: [&str; 28] = ["ps", "pr", "pl", "bl", "td", "tr", "tl", "dr", "dl", "kr", "kR", "kl", "gr", "gl", "mn", "fr", "fl", "sp", "sb", "st", "sk", "sR", "xɲ", "xt͡s", "xv", "xs", "xj", "ʙR"];
//const CONSONANTS_TRIO_ONSET: [&str; 0] = [];
const VOWELS: [&str; 6] = ["a", "e", "i", "y", "o", "u"];
const VOWELS_PAIRS: [&str; 13] = ["aa", "ee", "ii", "oo", "uu", "ae", "ai", "ao", "au", "eo", "eu", "oi", "ui"];
const CONSONANTS_CODA: [&str; 6] = ["p", "b", "t", "d", "k", "g"];

fn consonants_clusters() {
    let combinations : Vec<_> = CONSONANTS_ONSET.iter()
        .map(|&c| CONSONANTS_PAIRS_ONSET.iter().map(move |&d| c.to_owned() + d))
        .flatten()
        .collect();
    println!("Combinations");
    for elem in combinations {
        print!("{elem}, ");
    }
}

fn main() {
    consonants_clusters();
}
