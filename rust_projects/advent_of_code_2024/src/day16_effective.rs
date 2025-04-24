mod grid {
    //! Fast 2 dimensional Grid backed by a single `vec`. This module is designed to work with [`Point`].
    //!
    //! The traits [`Index`] and [`IndexMut`] are implemented for [`Point`] to allow usage like:
    //!
    //! ```
    //!   # use aoc::util::grid::Grid;
    //!   # use aoc::util::point::Point;
    //!
    //!   let mut grid = Grid::parse("1");
    //!   let point = Point::new(0, 0);
    //!
    //!   let foo = grid[point];
    //!   assert_eq!(foo, b'1');
    //!
    //!   grid[point] = foo + 1;
    //!   assert_eq!(grid[point], b'2');
    //! ```
    //!
    //! A convenience [`parse`] method creates a `Grid` directly from a 2 dimenionsal set of
    //! ASCII characters, a common occurence in Advent of Code inputs. The [`same_size_with`] function
    //! creates a grid of the same size, that can be used for in BFS algorithms for tracking visited
    //! location or for tracking cost in Djikstra.
    //!
    //! [`Point`]: crate::util::point
    //! [`parse`]: Grid::parse
    //! [`same_size_with`]: Grid::same_size_with
    use crate::point::*;
    use std::ops::{Index, IndexMut};

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Grid<T> {
        pub width: i32,
        pub height: i32,
        pub bytes: Vec<T>,
    }

    impl Grid<u8> {
        #[inline]
        pub fn parse(input: &str) -> Self {
            let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
            let width = raw[0].len() as i32;
            let height = raw.len() as i32;
            let mut bytes = Vec::with_capacity((width * height) as usize);
            raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
            Grid {
                width,
                height,
                bytes,
            }
        }

        pub fn print(&self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    let point = Point::new(x, y);
                    print!("{}", self[point] as char);
                }
                println!();
            }
            println!();
        }
    }

    impl<T: Copy + PartialEq> Grid<T> {
        #[inline]
        pub fn find(&self, needle: T) -> Option<Point> {
            let to_point = |index| {
                let x = (index as i32) % self.width;
                let y = (index as i32) / self.width;
                Point::new(x, y)
            };
            self.bytes.iter().position(|&h| h == needle).map(to_point)
        }
    }

    impl<T: Copy> Grid<T> {
        pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
            Grid {
                width,
                height,
                bytes: vec![value; (width * height) as usize],
            }
        }
    }

    impl<T> Grid<T> {
        #[inline]
        pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
            Grid {
                width: self.width,
                height: self.height,
                bytes: vec![value; (self.width * self.height) as usize],
            }
        }

        #[inline]
        pub fn contains(&self, point: Point) -> bool {
            point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
        }
    }

    impl<T> Index<Point> for Grid<T> {
        type Output = T;

        #[inline]
        fn index(&self, index: Point) -> &Self::Output {
            &self.bytes[(self.width * index.y + index.x) as usize]
        }
    }

    impl<T> IndexMut<Point> for Grid<T> {
        #[inline]
        fn index_mut(&mut self, index: Point) -> &mut Self::Output {
            &mut self.bytes[(self.width * index.y + index.x) as usize]
        }
    }
}
mod point {
    //! Comprehensive 2 dimensional point implementation. This class is designed to work together
    //! with the [`Grid`] class.
    //!
    //! A common theme in Advent of Code is operations in 2 dimensions. This module provides a
    //! [`Point`] struct along with implementations of several of the [`std::ops`] traits to support
    //! operator overloading, that allows shorthand expressions such as:
    //!
    //! ```
    //!   # use aoc::util::point::Point;
    //!
    //!   let a = Point::new(1, 2);
    //!   let b = Point::new(3, 4);
    //!   let k = 2;
    //!
    //!   assert_eq!(a + b, Point::new(4, 6));
    //!   assert_eq!(a - b, Point::new(-2, -2));
    //!   assert_eq!(a * k, Point::new(2, 4));
    //! ```
    //!
    //! Additionally there are [`clockwise`] and [`counter_clockwise`] functions for 90 degree rotations
    //! and a [`manhattan`] function for the
    //! [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between 2 points.
    //!
    //! [`clockwise`]: Point::clockwise
    //! [`counter_clockwise`]: Point::counter_clockwise
    //! [`manhattan`]: Point::manhattan
    //! [`Grid`]: crate::util::grid
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

    pub const ORIGIN: Point = Point::new(0, 0);
    pub const UP: Point = Point::new(0, -1);
    pub const DOWN: Point = Point::new(0, 1);
    pub const LEFT: Point = Point::new(-1, 0);
    pub const RIGHT: Point = Point::new(1, 0);
    pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
    // Left to right and top to bottom.
    pub const DIAGONAL: [Point; 8] = [
        Point::new(-1, -1),
        UP,
        Point::new(1, -1),
        LEFT,
        RIGHT,
        Point::new(-1, 1),
        DOWN,
        Point::new(1, 1),
    ];

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        #[inline]
        #[must_use]
        pub const fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }

        #[inline]
        #[must_use]
        pub fn clockwise(self) -> Self {
            Point::new(-self.y, self.x)
        }

        #[inline]
        #[must_use]
        pub fn counter_clockwise(self) -> Self {
            Point::new(self.y, -self.x)
        }

        #[inline]
        #[must_use]
        pub fn manhattan(self, other: Self) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }

        #[inline]
        #[must_use]
        pub fn signum(self, other: Self) -> Self {
            Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
        }
    }

    impl From<u8> for Point {
        #[inline]
        #[must_use]
        fn from(value: u8) -> Self {
            match value {
                b'^' | b'U' => UP,
                b'v' | b'D' => DOWN,
                b'<' | b'L' => LEFT,
                b'>' | b'R' => RIGHT,
                _ => unreachable!(),
            }
        }
    }

    impl Hash for Point {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_i32(self.x as i32);
            state.write_i32(self.y as i32);
        }
    }

    impl Add for Point {
        type Output = Self;

        #[inline]
        #[must_use]
        fn add(self, rhs: Self) -> Self {
            Point::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl AddAssign for Point {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl Mul<i32> for Point {
        type Output = Self;

        #[inline]
        #[must_use]
        fn mul(self, rhs: i32) -> Self {
            Point::new(self.x * rhs, self.y * rhs)
        }
    }

    impl Sub for Point {
        type Output = Self;

        #[inline]
        #[must_use]
        fn sub(self, rhs: Self) -> Self {
            Point::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl SubAssign for Point {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }
}

// # Reindeer Maze
//
// Solves part one and part two simultaneously.
//
// Part one is a normal [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
// search from start to end.
//
// Part two is a a BFS *backwards* from the end to the finish, tracing the cost exactly
// to find all possible paths. This reuses the cost information from the Dijkstra without
// requiring any extra state keeping for the paths.
//
// To speed things up even further we use a trick. Classic Dijkstra uses a generic priority queue
// that can be implemented in Rust using a [`BinaryHeap`]. However the total cost follows a
// strictly increasing order in a constrained range of values, so we can use a much faster
// [bucket queue](https://en.wikipedia.org/wiki/Bucket_queue). The maximum possible increase is
// 1000 so we need 1001 buckets.
//
// [`BinaryHeap`]: std::collections::BinaryHeap
use grid::*;
use point::*;
use std::collections::VecDeque;
use std::env;

type Input = (i32, usize);

/// Clockwise order starting with facing right.
const DIRECTIONS: [Point; 4] = [RIGHT, DOWN, LEFT, UP];

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Forwards Dijkstra
    let mut buckets = vec![Vec::new(); 1001];
    // State is `(position, direction)`.
    let mut seen = grid.same_size_with([i32::MAX; 4]);
    let mut cost = 0;
    let mut lowest = i32::MAX;

    buckets[0].push((start, 0));
    seen[start][0] = 0;

    while lowest == i32::MAX {
        let index = (cost % 1001) as usize;

        while let Some((position, direction)) = buckets[index].pop() {
            // Once we find the end node then stop. All paths of the same cost must be in
            // this bucket, so have already been accounted for.
            if position == end {
                lowest = cost;
                break;
            }

            // -1.rem_euclid(4) = 3
            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (position + DIRECTIONS[direction], direction, cost + 1),
                (position, left, cost + 1000),
                (position, right, cost + 1000),
            ];

            for (next_position, next_direction, next_cost) in next {
                if grid[next_position] != b'#' && next_cost < seen[next_position][next_direction] {
                    // Find the next bucket.
                    let index = (next_cost % 1001) as usize;
                    buckets[index].push((next_position, next_direction));
                    seen[next_position][next_direction] = next_cost;
                }
            }
        }

        cost += 1;
    }

    // Backwards BFS
    let mut todo = VecDeque::new();
    let mut path = grid.same_size_with(false);

    // Lowest paths can arrive at end node in multiple directions.
    for direction in 0..4 {
        if seen[end][direction] == lowest {
            todo.push_back((end, direction, lowest));
        }
    }

    while let Some((position, direction, cost)) = todo.pop_front() {
        path[position] = true;
        if position == start {
            continue;
        }

        // Reverse direction and subtract cost.
        let left = (direction + 3) % 4;
        let right = (direction + 1) % 4;
        let next = [
            (position - DIRECTIONS[direction], direction, cost - 1),
            (position, left, cost - 1000),
            (position, right, cost - 1000),
        ];

        for (next_position, next_direction, next_cost) in next {
            // Trace our cost step by step so it will exactly match possible paths.
            if next_cost == seen[next_position][next_direction] {
                todo.push_back((next_position, next_direction, next_cost));
                // Set cost back to `i32::MAX` to prevent redundant path explorations.
                seen[next_position][next_direction] = i32::MAX;
            }
        }
    }

    (lowest, path.bytes.iter().filter(|&&b| b).count())
}

pub fn part1(input: &str) -> i32 {
    parse(input).0
}

pub fn part2(input: &str) -> usize {
    parse(input).1
}
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let input = std::fs::read_to_string("data/day16.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
