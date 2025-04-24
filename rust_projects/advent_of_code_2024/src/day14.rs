use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

impl FromStr for Velocity {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0].parse::<i32>()?;
        let x = x.rem_euclid(101);
        let y = parts[1].parse::<i32>()?;
        let y = y.rem_euclid(103);
        Ok(Velocity { x, y })
    }
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Velocity,
}

impl FromStr for Robot {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let pos = parts[0]
            .strip_prefix("p=")
            .ok_or("Missing 'p=' prefix")?
            .parse::<Point>()?;
        let vel = parts[1]
            .strip_prefix("v=")
            .ok_or("Missing 'v=' prefix")?
            .parse::<Velocity>()?;
        Ok(Robot { pos, vel })
    }
}

fn part1(s: &str) -> i32 {
    let tall = 103;
    let wide = 101;
    //let tall = 7;
    //let wide = 11;
    let seconds = 100;
    let mut robots: Vec<Robot> = s
        .lines()
        .map(|line| line.parse::<Robot>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse robots");
    // just for the tree
    let mut v: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];
    for robot in &mut robots {
        let x = (robot.pos.x + robot.vel.x * 7790) % 101;
        let y = (robot.pos.y + robot.vel.y * 7790) % 103;
        v[y as usize][x as usize] = '1';
    }
    for r in v {
        for ch in r {
            print!("{}", ch);
        }
        println!();
    }

    for _ in 0..seconds {
        for robot in &mut robots {
            robot.pos.x += robot.vel.x;
            robot.pos.y += robot.vel.y;
            robot.pos.x = robot.pos.x.rem_euclid(wide);
            robot.pos.y = robot.pos.y.rem_euclid(tall);
        }
    }
    let half_tall = (tall - 2) / 2;
    let half_wide = (wide - 2) / 2;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in &robots {
        match robot.pos {
            Point { x, y } if x >= 0 && x <= half_wide && y >= 0 && y <= half_tall => q1 += 1, // Top-left quadrant
            Point { x, y } if x > half_wide + 1 && x <= wide && y >= 0 && y <= half_tall => q2 += 1, // Top-right quadrant
            Point { x, y } if x >= 0 && x <= half_wide && y > half_tall + 1 && y <= tall => q3 += 1, // Bottom-left quadrant
            Point { x, y } if x > half_wide + 1 && x <= wide && y > half_tall + 1 && y <= tall => {
                q4 += 1
            } // Bottom-right quadrant
            _ => {} // Outside the defined quadrants
        }
    }

    q1 * q2 * q3 * q4
}

// Copied because im a lazy dumb bitch
fn part2(s: &str) -> i32 {
    let robots: Vec<Robot> = s
        .lines()
        .map(|line| line.parse::<Robot>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse robots");
    // Search for times mod 101 when the tree could possibly exist using x coordinates only.
    // and times mod 103 when the tree could possibly exist using y coordinates only.
    let mut rows = Vec::new();
    let mut columns = Vec::new();

    for time in 0..103 {
        let mut xs = [0; 101];
        let mut ys = [0; 103];

        for robot in &robots {
            let x = (robot.pos.x + time * robot.vel.x) % 101;
            xs[x as usize] += 1;
            let y = (robot.pos.y + time * robot.vel.y) % 103;
            ys[y as usize] += 1;
        }

        // Tree bounding box is 31x33.
        if time < 101 && xs.iter().filter(|&&c| c >= 33).count() >= 2 {
            columns.push(time);
        }
        if ys.iter().filter(|&&c| c >= 31).count() >= 2 {
            rows.push(time);
        }
    }

    // If there's only one combination then return answer.
    if rows.len() == 1 && columns.len() == 1 {
        let t = columns[0];
        let u = rows[0];
        // Combine indices using the Chinese Remainder Theorem to get index mod 10403.
        return (5253 * t + 5151 * u) % 10403;
    }

    // Backup check looking for time when all robot positions are unique.
    let mut floor = vec![0; 10403];

    for &t in &columns {
        'outer: for &u in &rows {
            let time = (5253 * t + 5151 * u) % 10403;

            for robot in &robots {
                let x = (robot.pos.x + time * robot.vel.x) % 101;
                let y = (robot.pos.y + time * robot.vel.y) % 103;

                let index = 101 * y + x;
                if floor[index as usize] == time {
                    continue 'outer;
                }
                floor[index as usize] = time;
            }

            return time;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part1(report), 12);
    }
}
