use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> usize {
    let mut g = Grid::new(input);
    while g.update_guard().is_some() {}
    g.grid.into_iter().filter(|&x| x == b'X').count()
}
fn part2(input: &'static str) -> usize {
    let mut g = Grid::new(input);
    let mut run = true;
    let mut visited = HashSet::new();
    let mut cache = HashSet::new();
    while run {
        let next_pos = g.current + g.direction;
        if g.get(&next_pos).is_some_and(|c| c != b'#')
            && (!cache.contains(&next_pos))
            && g.will_loop(&mut visited)
        {
            cache.insert(next_pos);
        }
        run = g.update_guard().is_some();
    }
    cache.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
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

struct Grid {
    n: isize,
    grid: Vec<u8>,
    init: Coord,
    current: Coord,
    direction: Coord,
}

impl Grid {
    fn new(input: &'static str) -> Self {
        let input = input.trim();
        let n = input.lines().count();
        let mut grid = Vec::<u8>::with_capacity(n * n);
        let mut init = Coord::default();
        for (y, line) in input.lines().enumerate() {
            let line = line.as_bytes();
            if let Some(x) = line.iter().position(|&x| x == b'^') {
                init = Coord([x as isize, y as isize]);
            }
            grid.extend(line);
        }
        let direction = Coord([0, -1]);
        let n = n as isize;
        let mut g = Grid {
            n,
            grid,
            init,
            current: init,
            direction,
        };
        g.set(&init, b'X');
        g
    }
    fn will_loop(&mut self, set: &mut HashSet<[Coord; 2]>) -> bool {
        let new_wall = self.current + self.direction;
        self.set(&new_wall, b'#');
        let mut cur = self.init;
        let mut dir = Coord([0, -1]);
        let mut looping = true;
        while set.insert([cur, dir]) {
            if let Some(c) = self.next_wall(cur, dir) {
                cur = c - dir;
                dir = dir.rotate_right();
            } else {
                looping = false;
                break;
            }
        }
        self.set(&new_wall, b'.');
        set.clear();
        looping
    }
    fn next_wall(&self, mut coord: Coord, dir: Coord) -> Option<Coord> {
        while self.get(&coord)? != b'#' {
            coord = coord + dir;
        }
        Some(coord)
    }
    fn update_guard(&mut self) -> Option<bool> {
        let new_pos = self.current + self.direction;
        Some(if self.get(&new_pos)? == b'#' {
            self.direction = self.direction.rotate_right();
            true
        } else {
            self.set(&new_pos, b'X');
            self.current = new_pos;
            false
        })
    }
    fn get(&self, coord: &Coord) -> Option<u8> {
        let [x, y] = coord.0;
        if (0..self.n).contains(&x) && (0..self.n).contains(&y) {
            Some(self.grid[(x + self.n * y) as usize])
        } else {
            None
        }
    }
    fn set(&mut self, coord: &Coord, chr: u8) {
        let [x, y] = coord.0;
        if (0..self.n).contains(&x) && (0..self.n).contains(&y) {
            self.grid[(x + self.n * y) as usize] = chr;
        }
    }
}
#[cfg(test)]
const EXAMPLE: &'static str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 41);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 6);
}
