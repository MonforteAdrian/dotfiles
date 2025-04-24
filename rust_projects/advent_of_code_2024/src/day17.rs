use rayon::prelude::*;
use z3::ast::{Ast, BV};

fn main() {
    let input = std::fs::read_to_string("data/day17.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}
fn parse_input(s: &str) -> ((i64, i64, i64), Vec<i64>) {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let registers: Vec<i64> = parts[0]
        .lines()
        .map(|line| {
            let registers = line.split_whitespace();
            registers.last().unwrap().parse().unwrap()
        })
        .collect();
    let commands: Vec<&str> = parts[1].split_whitespace().collect();
    let commands: Vec<i64> = commands[1]
        .split(",")
        .map(|command| command.parse().unwrap())
        .collect();

    ((registers[0], registers[1], registers[2]), commands)
}

fn combo((a, b, c): (i64, i64, i64), operand: i64) -> i64 {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        x => x,
    }
}

fn part1(s: &str) -> String {
    let ((mut a, mut b, mut c), commands) = parse_input(s);
    let mut output = String::new();
    let mut i = 0;
    let mut increase: bool;
    while i < commands.len() - 1 {
        increase = true;
        let (opcode, operand) = (commands[i], commands[i + 1]);
        match opcode {
            0 => a /= 2_i64.pow(combo((a, b, c), operand) as u32),
            1 => b ^= operand,
            2 => b = combo((a, b, c), operand) % 8,
            3 => {
                if a != 0 {
                    increase = false;
                    i = operand as usize;
                }
            }
            4 => b ^= c,
            5 => {
                if !output.is_empty() {
                    output.push(',');
                }
                output.push_str(&(combo((a, b, c), operand) % 8).to_string())
            }
            6 => b = a / 2_i64.pow(combo((a, b, c), operand) as u32),
            7 => c = a / 2_i64.pow(combo((a, b, c), operand) as u32),
            _ => panic!(),
        }

        if increase {
            i += 2;
        }
    }
    output
}

fn part2(_: &str) -> i64 {
    let ctx = z3::Context::new(&z3::Config::new());
    let opt = z3::Optimize::new(&ctx);
    let s = BV::new_const(&ctx, "s", 64);
    let (mut a, mut b, mut c) = (
        s.clone(),
        BV::from_i64(&ctx, 0, 64),
        BV::from_i64(&ctx, 0, 64),
    );
    for x in [2, 4, 1, 6, 7, 5, 4, 4, 1, 7, 0, 3, 5, 5, 3, 0] {
        b = &a & &BV::from_i64(&ctx, 7, 64);
        b = b ^ &BV::from_i64(&ctx, 6, 64);
        c = a.bvlshr(&b);
        b = b ^ c;
        b = b ^ &BV::from_i64(&ctx, 7, 64);
        a = a.bvlshr(&BV::from_i64(&ctx, 3, 64));
        opt.assert(&(&b & &BV::from_i64(&ctx, 7, 64))._eq(&BV::from_i64(&ctx, x, 64)));
    }
    opt.assert(&(a._eq(&BV::from_i64(&ctx, 0, 64))));
    opt.minimize(&s);
    assert_eq!(opt.check(&[]), z3::SatResult::Sat);
    let res = opt
        .get_model()
        .unwrap()
        .eval(&s, true)
        .unwrap()
        .as_i64()
        .unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part1(report), "4,6,3,5,6,3,5,2,1,0");
    }
}
