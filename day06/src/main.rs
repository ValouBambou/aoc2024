use std::{collections::HashSet, fmt::Debug, ops::Add};

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
    let mut possible = HashSet::new();
    while run {
        let next_pos = g.current + g.direction;
        // TODO: maybe we can filter more to reduce time in cycle detection after
        if g.get(&next_pos).is_some_and(|c| c != b'#') && g.next_wall_right().is_some() {
            possible.insert(next_pos);
        }
        run = g.update_guard().is_some();
    }
    let mut visited = HashSet::new();
    possible
        .into_iter()
        .filter(|new_wall| g.will_loop(new_wall, &mut visited))
        .count()
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

struct Grid {
    n: isize,
    grid: Vec<u8>,
    init: Coord,
    current: Coord,
    direction: Coord,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Grid {}x{} cur x {} y {} dir x {} y {}",
            self.n,
            self.n,
            self.current.0[0],
            self.current.0[1],
            self.direction.0[0],
            self.direction.0[1]
        )?;
        for y in 0..self.n {
            for x in 0..self.n {
                write!(f, "{}", self.get(&Coord([x, y])).unwrap() as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &'static str) -> Self {
        let input = input.trim();
        let n = input.lines().count();
        let mut grid = Vec::<u8>::with_capacity(n * n);
        let mut current = Coord::default();
        for (y, line) in input.lines().enumerate() {
            let line = line.as_bytes();
            if let Some(x) = line.iter().position(|&x| x == b'^') {
                current = Coord([x as isize, y as isize]);
            }
            grid.extend(line);
        }
        let direction = Coord([0, -1]);
        let n = n as isize;
        let mut g = Grid {
            n,
            grid,
            init: current,
            current,
            direction,
        };
        g.set(&current, b'X');
        g
    }
    fn will_loop(&mut self, coord: &Coord, set: &mut HashSet<[isize; 4]>) -> bool {
        self.set(coord, b'#');
        let mut i = Some(false);
        while let Some(b) = i {
            if b && (!set.insert([
                self.current.0[0],
                self.current.0[1],
                self.direction.0[0],
                self.direction.0[1],
            ])) {
                break;
            }
            i = self.update_guard();
        }
        self.set(coord, b'.');
        self.reset();
        set.clear();
        i.is_some()
    }
    fn next_wall_right(&self) -> Option<Coord> {
        let mut coord = self.current;
        let dir = self.direction.rotate_right();
        while self.get(&coord)? != b'#' {
            coord = coord + dir;
        }
        Some(coord)
    }
    fn reset(&mut self) {
        self.current = self.init;
        self.direction = Coord([0, -1]);
        self.grid
            .iter_mut()
            .filter(|x| **x == b'X')
            .for_each(|x| *x = b'.');
        let init = self.init;
        self.set(&init, b'X');
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
