use itertools::Itertools;

const NUM_CHARS: usize = 11;

const LUT3: [u64; 121] = [
    1, 25, 12, 19, 26, 13, 20, 27, 14, 21, 10, 21, 1, 10, 11, 12, 19, 20, 13, 20, 21, 22, 16, 18,
    1, 10, 21, 12, 19, 22, 13, 20, 17, 21, 19, 18, 1, 22, 21, 12, 23, 22, 13, 16, 22, 16, 17, 18,
    1, 10, 11, 12, 19, 20, 23, 17, 21, 16, 17, 18, 1, 10, 21, 12, 19, 18, 22, 22, 21, 16, 19, 18,
    1, 22, 21, 12, 17, 23, 17, 18, 19, 16, 17, 18, 1, 10, 11, 24, 18, 22, 17, 18, 21, 16, 17, 18,
    1, 10, 19, 23, 23, 22, 17, 22, 21, 16, 19, 18, 1, 18, 18, 26, 21, 12, 27, 22, 13, 28, 23, 14,
    1,
];
const LUT26: [u64; 121] = [
    1,
    31420065369,
    14752615084,
    24095973437,
    31420065370,
    14752615085,
    24095973438,
    31420065371,
    14752615086,
    24095973439,
    14287938116,
    27052881363,
    1,
    14287938116,
    14287938117,
    14752615084,
    24095973437,
    24095973438,
    14752615085,
    24095973438,
    24095973439,
    27052881364,
    20790420654,
    22411052532,
    1,
    14287938116,
    28154654777,
    14752615084,
    24095973437,
    28154654778,
    14752615085,
    24095973438,
    22778092491,
    27622800565,
    22411052533,
    22411052532,
    1,
    28154654778,
    28154654777,
    14752615084,
    28154654779,
    28154654778,
    14752615085,
    20790420654,
    27052881364,
    20790420654,
    22778092491,
    22778092492,
    1,
    14287938116,
    14287938117,
    14752615084,
    24095973437,
    24095973438,
    27052881365,
    20790420655,
    27622800565,
    20790420654,
    22778092491,
    22411052532,
    1,
    14287938116,
    28154654777,
    14752615084,
    24095973437,
    22778092492,
    27622800566,
    27622800566,
    27622800565,
    20790420654,
    22411052533,
    22411052532,
    1,
    28154654778,
    28154654777,
    14752615084,
    20790420655,
    27052881365,
    20790420655,
    22778092492,
    22778092493,
    20790420654,
    22778092491,
    22778092492,
    1,
    14287938116,
    14287938117,
    27052881366,
    20790420656,
    27622800566,
    20790420655,
    22778092492,
    27622800565,
    20790420654,
    22778092491,
    22411052532,
    1,
    14287938116,
    22778092493,
    27622800567,
    27622800567,
    27622800566,
    20790420655,
    27622800566,
    27622800565,
    20790420654,
    22411052533,
    22411052532,
    1,
    20790420656,
    22411052532,
    31420065370,
    28154654777,
    14752615084,
    31420065371,
    28154654778,
    14752615085,
    31420065372,
    28154654779,
    14752615086,
    1,
];

fn encode(c: char) -> u8 {
    let o = c as u8;
    o % 16 + (o / 64) * 9
}

//  there is a set of depth independent optimal moves => a LUT of the replacements can be built
//  since all moves are independent (i.e. their order does not matter) a state becomes a vector of moves
//  the replacement becomes a linear transform => all codes can be added beforehand and transformed as one
//  since the number of steps is fixed, we can precompute the matrix
//  the final count is just a dot with the vector of all 1s => precompute dot with matrix
//  => each part reduces to a single dot product
fn part1(data: &str) -> String {
    let mut counts = vec![0; NUM_CHARS * NUM_CHARS];

    for inp in data.lines() {
        let inp = inp.trim();
        let num_val = inp[..inp.len() - 1].parse::<u64>().unwrap();
        //let num_val=ULINT::parse_str_radix(&inp[..inp.len()-1],10);

        for (c, cn) in "A".chars().chain(inp.chars()).map(encode).tuple_windows() {
            counts[(c as usize) * NUM_CHARS + (cn as usize)] += num_val;
        }
    }

    let p1 = counts
        .iter()
        .zip(LUT3.iter())
        .map(|(x, y)| x * y)
        .sum::<u64>();
    p1.to_string()
}

fn part2(data: &str) -> String {
    let mut counts = vec![0; NUM_CHARS * NUM_CHARS];

    for inp in data.lines() {
        let inp = inp.trim();
        let num_val = inp[..inp.len() - 1].parse::<u64>().unwrap();
        //let num_val=ULINT::parse_str_radix(&inp[..inp.len()-1],10);

        for (c, cn) in "A".chars().chain(inp.chars()).map(encode).tuple_windows() {
            counts[(c as usize) * NUM_CHARS + (cn as usize)] += num_val;
        }
    }

    let p2 = counts
        .iter()
        .zip(LUT26.iter())
        .map(|(x, y)| x * y)
        .sum::<u64>();
    p2.to_string()
}
fn main() {
    let input = std::fs::read_to_string("data/day21.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let report = "029A\n980A\n179A\n456A\n379A";
        assert_eq!(part2(report), "126384");
    }
}
