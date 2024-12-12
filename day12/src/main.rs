use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    part_x(input, |borders| borders.len())
}
fn part2(input: &'static str) -> usize {
    part_x(input, |borders| {
        let mut nsides = 0;
        while let Some(&(coord, dir)) = borders.iter().next() {
            borders.remove(&(coord, dir));
            let orth_dir = dir.rotate_right();
            for orth in [orth_dir, -orth_dir] {
                let mut nei = (coord + orth, dir);
                while borders.contains(&nei) {
                    borders.remove(&nei);
                    nei.0 = nei.0 + orth;
                }
            }
            nsides += 1;
        }
        nsides
    })
}
fn part_x<F>(input: &'static str, f: F) -> usize
where
    F: Fn(&mut HashSet<(Coord, Coord)>) -> usize,
{
    let mut g = Grid::new(input);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut borders = HashSet::new();
    let mut total = 0;
    while let Some((&coord, &letter)) = g.map.iter().next() {
        let (score, area) = g.tag_region(coord, letter, &mut queue, &mut visited, &mut borders, &f);
        total += score * area;
    }
    total
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
struct Coord([isize; 2]);

impl Coord {
    fn rotate_right(&self) -> Self {
        Coord(match self.0 {
            [0, y] => [-y, 0],
            [x, 0] => [0, x],
            c => panic!("rotate {:?} not supported", c),
        })
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

struct Grid {
    map: HashMap<Coord, u8>,
}

const DIRECTIONS: [Coord; 4] = [Coord([1, 0]), Coord([-1, 0]), Coord([0, 1]), Coord([0, -1])];

impl Grid {
    fn tag_region<F: Fn(&mut HashSet<(Coord, Coord)>) -> usize>(
        &mut self,
        start: Coord,
        letter: u8,
        queue: &mut VecDeque<Coord>,
        visited: &mut HashSet<Coord>,
        borders: &mut HashSet<(Coord, Coord)>,
        score_fn: F,
    ) -> (usize, usize) {
        self.map.remove(&start);
        queue.push_front(start);
        visited.insert(start);
        while let Some(coord) = queue.pop_back() {
            for dir in DIRECTIONS {
                let nei = coord + dir;
                if self.map.get(&nei).is_some_and(|&nei_l| nei_l == letter) {
                    self.map.remove(&nei);
                    queue.push_front(nei);
                    visited.insert(nei);
                }
                if !visited.contains(&nei) {
                    borders.insert((nei, dir));
                }
            }
        }
        let area = visited.len();
        let score = score_fn(borders);
        visited.clear();
        borders.clear();
        (score, area)
    }
    fn new(input: &'static str) -> Self {
        Grid {
            map: input
                .trim()
                .lines()
                .enumerate()
                .flat_map(move |(y, line)| {
                    line.as_bytes()
                        .iter()
                        .enumerate()
                        .map(move |(x, &c)| (Coord([x as isize, y as isize]), c))
                })
                .collect(),
        }
    }
}
#[cfg(test)]
const EXAMPLE: &'static str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 1930);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 1206);
}
