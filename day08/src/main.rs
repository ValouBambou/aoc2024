use std::{
    collections::HashMap,
    ops::{Add, Neg, Sub},
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> usize {
    part_x(input, |c1, c2, mat| {
        let dist = c2 - c1;
        mat.set(c1 - dist, b'#');
        mat.set(c2 + dist, b'#');
    })
}
fn part2(input: &'static str) -> usize {
    part_x(input, |c1, c2, mat| {
        let dist = (c2 - c1).normalize();
        let mut tag_antinodes = |init: Coord, dir: Coord| {
            let mut cur = init;
            while mat.set(cur, b'#') {
                cur = cur + dir;
            }
        };
        tag_antinodes(c1, -dist);
        tag_antinodes(c1, dist);
    })
}
fn part_x<F: FnMut(Coord, Coord, &mut CharGrid)>(input: &'static str, mut func: F) -> usize {
    let mut mat = CharGrid::new(input);
    let mut antenas = HashMap::new();
    for y in 0..mat.n {
        for x in 0..mat.n {
            let coord = Coord([x, y]);
            let antena = mat.buf[(x + mat.n * y) as usize];
            if antena != b'.' {
                antenas
                    .entry(antena)
                    .or_insert(Vec::<Coord>::new())
                    .push(coord);
            }
        }
    }
    for (_, coords) in antenas {
        for (i, &c1) in coords.iter().enumerate() {
            for &c2 in coords.iter().take(i) {
                func(c1, c2, &mut mat);
            }
        }
    }
    mat.buf.into_iter().filter(|&c| c == b'#').count()
}
struct CharGrid {
    n: isize,
    buf: Vec<u8>,
}
impl CharGrid {
    fn new(input: &'static str) -> Self {
        let input = input.trim();
        let n = input.lines().count() as isize;
        let buf = input
            .lines()
            .flat_map(|s| s.as_bytes().iter().map(|&c| c))
            .collect::<Vec<u8>>();
        CharGrid { n, buf }
    }
    fn set(&mut self, coord: Coord, chr: u8) -> bool {
        let [x, y] = coord.0;
        if (0..self.n).contains(&x) && (0..self.n).contains(&y) {
            self.buf[(x + self.n * y) as usize] = chr;
            true
        } else {
            false
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([isize; 2]);

impl Coord {
    fn normalize(&self) -> Coord {
        let [x, y] = self.0;
        let (min, max) = if x.abs() > y.abs() { (y, x) } else { (x, y) };
        if min != 0 && max % min == 0 {
            Coord([x / min, y / min])
        } else {
            *self
        }
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1]])
    }
}
impl Neg for Coord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Coord([-self.0[0], -self.0[1]])
    }
}

#[cfg(test)]
const EXAMPLE: &'static str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 14);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 34);
}
