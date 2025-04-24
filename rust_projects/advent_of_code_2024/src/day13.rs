use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}
struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    prize_x: i64,
    prize_y: i64,
}

impl ClawMachine {
    fn min_tokens(&self) -> i64 {
        let (ax, ay, bx, by, prize_x, prize_y) = (
            self.ax,
            self.ay,
            self.bx,
            self.by,
            self.prize_x,
            self.prize_y,
        );

        let det = ax * by - ay * bx;
        let a = (prize_x * by - prize_y * bx) / det;
        let b = (ax * prize_y - ay * prize_x) / det;
        if (ax * a + bx * b, ay * a + by * b) == (prize_x, prize_y) {
            a * 3 + b
        } else {
            0
        }
    }
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != 3 {
            return Err("Invalid input format".to_string());
        }

        let parse_button = |line: &str| -> Result<(i64, i64), String> {
            let parts: Vec<&str> = line
                .split(':')
                .nth(1)
                .ok_or("Missing button data")?
                .split(',')
                .map(|x| x.trim())
                .collect();
            if parts.len() != 2 {
                return Err("Invalid button format".to_string());
            }
            let x = parts[0]
                .trim_start_matches("X+")
                .parse()
                .map_err(|_| "Invalid X value")?;
            let y = parts[1]
                .trim_start_matches("Y+")
                .parse()
                .map_err(|_| "Invalid Y value")?;
            Ok((x, y))
        };

        let (ax, ay) = parse_button(lines[0])?;
        let (bx, by) = parse_button(lines[1])?;

        let prize_parts: Vec<&str> = lines[2]
            .split(':')
            .nth(1)
            .ok_or("Missing prize data")?
            .split(',')
            .map(|x| x.trim())
            .collect();
        if prize_parts.len() != 2 {
            return Err("Invalid prize format".to_string());
        }
        let prize_x = prize_parts[0]
            .trim_start_matches("X=")
            .parse()
            .map_err(|_| "Invalid prize X")?;
        let prize_y = prize_parts[1]
            .trim_start_matches("Y=")
            .parse()
            .map_err(|_| "Invalid prize Y")?;

        Ok(ClawMachine {
            ax,
            ay,
            bx,
            by,
            prize_x,
            prize_y,
        })
    }
}

fn part1(s: &str) -> i64 {
    let mut total_cost = 0;

    let machines: Vec<ClawMachine> = s
        .split("\n\n")
        .map(|chunk| chunk.parse::<ClawMachine>())
        .collect::<Result<_, _>>()
        .expect("Failed to parse input");

    for machine in &machines {
        total_cost += machine.min_tokens();
    }
    total_cost
}

fn part2(s: &str) -> i64 {
    const OFFSET: i64 = 10_000_000_000_000;

    let mut machines: Vec<ClawMachine> = s
        .split("\n\n")
        .map(|chunk| chunk.parse::<ClawMachine>())
        .collect::<Result<_, _>>()
        .expect("Failed to parse input");

    // Update the prize positions with the offset
    for machine in &mut machines {
        machine.prize_x += OFFSET;
        machine.prize_y += OFFSET;
    }

    let mut total_cost = 0;

    for machine in &machines {
        total_cost += machine.min_tokens();
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part1(report), 480);
    }
}
