use core::panic;
use std::{collections::VecDeque, ops::Add};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> usize {
    part_x(input, false)
}
fn part2(input: &'static str) -> usize {
    part_x(input, true)
}
fn part_x(input: &'static str, part2: bool) -> usize {
    let mut g = Grid::new(input, part2);
    let mut visited = VecDeque::new();
    while g.move_robot(&mut visited).is_some() {}
    let n = g.nx as usize;
    let target = if part2 { b'[' } else { b'O' };
    g.grid
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            (x == target).then(|| {
                let [x, y] = [i % n, i / n];
                100 * y + x
            })
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([isize; 2]);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

struct Grid {
    nx: isize,
    ny: isize,
    grid: Vec<u8>,
    current: Coord,
    instructions: VecDeque<u8>,
}

impl Grid {
    fn new(input: &'static str, part2: bool) -> Self {
        let input = input.trim();
        let (p1, p2) = input.split_once("\n\n").unwrap();
        let ny = p1.lines().count();
        let nx = ny + (part2 as usize) * ny;
        let grid = if part2 {
            p1.lines()
                .flat_map(|line| {
                    line.as_bytes().iter().flat_map(|&x| match x {
                        b'@' => [b'@', b'.'],
                        b'#' => [b'#', b'#'],
                        b'.' => [b'.', b'.'],
                        b'O' => [b'[', b']'],
                        _ => panic!("wtf"),
                    })
                })
                .collect::<Vec<u8>>()
        } else {
            p1.lines()
                .flat_map(|line| line.as_bytes().iter().map(|x| *x))
                .collect::<Vec<u8>>()
        };
        let i = grid.iter().position(|&c| c == b'@').unwrap();
        let init = Coord([(i % nx) as isize, (i / nx) as isize]);
        let mut g = Grid {
            nx: nx as isize,
            ny: ny as isize,
            grid,
            current: init,
            instructions: p2
                .lines()
                .flat_map(|s| s.as_bytes().iter().map(|c| *c))
                .collect(),
        };
        g.set(&init, b'.');
        g
    }
    fn get(&self, coord: &Coord) -> Option<u8> {
        let [x, y] = coord.0;
        if (0..self.nx).contains(&x) && (0..self.ny).contains(&y) {
            Some(self.grid[(x + self.nx * y) as usize])
        } else {
            None
        }
    }
    fn set(&mut self, coord: &Coord, chr: u8) {
        let [x, y] = coord.0;
        if (0..self.nx).contains(&x) && (0..self.ny).contains(&y) {
            self.grid[(x + self.nx * y) as usize] = chr;
        }
    }
    fn move_robot(&mut self, visited: &mut VecDeque<Coord>) -> Option<()> {
        let instr = self.instructions.pop_front()?;
        let dir = direction(instr);
        let next_pos = self.current + dir;
        let object = self.get(&next_pos)?;
        match object {
            b'.' => {
                self.current = next_pos;
            }
            b'#' => {}
            b'O' => {
                // part 1 box are 1 char
                let mut box_pos = next_pos;
                while self.get(&box_pos)? == b'O' {
                    box_pos = box_pos + dir;
                }
                if self.get(&box_pos)? == b'#' {
                    return Some(());
                }
                self.set(&box_pos, b'O');
                self.set(&next_pos, b'.');
                self.current = next_pos;
            }
            object if dir.0[0] != 0 => {
                // part 2 box is [] and move horizontal
                let mut box_pos = next_pos;
                let mut next_char = object;
                while next_char == b'[' || next_char == b']' {
                    box_pos = box_pos + dir;
                    debug_assert_eq!(self.get(&box_pos)?, closing_char(next_char));
                    box_pos = box_pos + dir;
                    next_char = self.get(&box_pos)?;
                }
                if self.get(&box_pos)? == b'#' {
                    return Some(());
                }
                let mut box_pos = next_pos;
                let mut next_char = object;
                while next_char == b'[' || next_char == b']' {
                    self.set(&box_pos, closing_char(next_char));
                    box_pos = box_pos + dir;
                    next_char = self.get(&box_pos)?;
                }
                self.set(&box_pos, closing_char(object));
                self.set(&next_pos, b'.');
                self.current = next_pos;
            }
            _ => {
                // part 2 box is [] and move vertical
                // complex staircases should move all at once
                visited.clear();
                if self.rec_move(next_pos, dir, visited) {
                    for k in visited.iter() {
                        self.set(&(*k + dir), self.get(k)?);
                        self.set(k, b'.');
                    }
                    self.current = next_pos;
                }
            }
        }
        Some(())
    }
    fn rec_move(&self, pos: Coord, dir: Coord, visited: &mut VecDeque<Coord>) -> bool {
        if visited.contains(&pos) {
            return true;
        }
        match self.get(&pos).unwrap() {
            b'#' => false,
            b'.' => true,
            bracket => {
                let next = pos + dir;
                let orth = closing_direction(bracket);
                let next2 = next + orth;
                if self.rec_move(next, dir, visited) && self.rec_move(next2, dir, visited) {
                    visited.push_back(pos);
                    visited.push_back(pos + orth);
                    true
                } else {
                    false
                }
            }
        }
    }
}
fn closing_char(c: u8) -> u8 {
    match c {
        b'[' => b']',
        b']' => b'[',
        c => panic!("{c} cannot be closed"),
    }
}
fn closing_direction(c: u8) -> Coord {
    match c {
        b'[' => Coord([1, 0]),
        b']' => Coord([-1, 0]),
        c => panic!("{c} cannot be closed"),
    }
}
fn direction(cmd: u8) -> Coord {
    Coord(match cmd {
        b'>' => [1, 0],
        b'<' => [-1, 0],
        b'^' => [0, -1],
        b'v' => [0, 1],
        cmd => panic!("unsupported cmd {cmd}"),
    })
}
#[cfg(test)]
const EXAMPLE_LARGE: &'static str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[cfg(test)]
const EXAMPLE_SMALL: &'static str = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE_SMALL), 2028);
    assert_eq!(part1(EXAMPLE_LARGE), 10092);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE_LARGE), 9021);
}
