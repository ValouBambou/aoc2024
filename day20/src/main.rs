use std::{
    collections::{HashMap, VecDeque},
    ops::{Add, Mul},
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> u32 {
    part_x(input, 2)
}
fn part2(input: &'static str) -> u32 {
    part_x(input, 20)
}
fn part_x(input: &'static str, cheat_duration: u8) -> u32 {
    let mut g = Grid::new(input);
    let base_cost = g.dijsktra();
    let cheats = g.find_cheats(base_cost, cheat_duration);
    let mut debug_v = cheats.iter().collect::<Vec<_>>();
    debug_v.sort_by_key(|t| t.0);
    for (gain, n_cheats) in debug_v {
        println!("{n_cheats} cheats saving {gain} picoseconds");
    }

    cheats.into_values().sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([i16; 2]);
impl Coord {
    fn dist(&self, coord: Coord) -> u32 {
        self.0[0].abs_diff(coord.0[0]) as u32 + self.0[1].abs_diff(coord.0[1]) as u32
    }
}
const DIRECTIONS: [Coord; 4] = [Coord([1, 0]), Coord([-1, 0]), Coord([0, 1]), Coord([0, -1])];
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Mul<i16> for Coord {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        Coord([self.0[0] * rhs, self.0[1] * rhs])
    }
}

struct Grid {
    n: i16,
    grid: Vec<u8>,
    start: usize,
    end: usize,
}
impl Grid {
    fn new(input: &'static str) -> Self {
        let input = input.trim();
        let grid = input
            .lines()
            .flat_map(|l| l.as_bytes().iter())
            .cloned()
            .collect::<Vec<u8>>();
        Grid {
            n: input.lines().count() as i16,
            start: grid.iter().position(|&x| x == b'S').unwrap(),
            end: grid.iter().position(|&x| x == b'E').unwrap(),
            grid,
        }
    }
    fn idx(&self, coord: &Coord) -> usize {
        let [x, y] = coord.0;
        x as usize + (self.n as usize) * (y as usize)
    }
    fn pos(&self, idx: usize) -> Coord {
        let n = self.n as usize;
        debug_assert!(idx < n * n);
        Coord([(idx % n) as i16, (idx / n) as i16])
    }
    fn find_cheats(&mut self, base_score: u32, cheat_duration: u8) -> HashMap<u32, u32> {
        let mut visited = vec![false; self.grid.len()];
        let mut cheats = HashMap::new();
        let node = MazeNode2 {
            cost: 0,
            position: self.pos(self.start),
            cheat: Cheat::Usable,
        };
        visited[self.start] = true;
        self.dfs_cheat(node, &mut cheats, &mut visited, base_score, cheat_duration);
        let mut occurences = HashMap::new();
        for v in cheats.into_values() {
            *occurences.entry(v).or_insert(0) += 1;
        }
        occurences
    }
    fn cheat_neighbors(&self, cheat_duration: u8, cur: Coord) -> Vec<(Coord, usize)> {
        let cheat_duration = cheat_duration as i16;
        let mut res = vec![];
        for ix in (-cheat_duration)..=cheat_duration {
            for iy in (-cheat_duration)..=cheat_duration {
                if ix + iy == 0 || ix.abs() + iy.abs() > cheat_duration {
                    continue;
                }
                let nei = cur + Coord([ix, iy]);
                let idx;
                if (0..self.n).contains(&nei.0[0])
                    && (0..self.n).contains(&nei.0[1])
                    && self.grid[{
                        idx = self.idx(&nei);
                        idx
                    }] != b'#'
                {
                    res.push((nei, idx));
                }
            }
        }
        res
    }
    fn dfs_cheat(
        &mut self,
        cur: MazeNode2,
        cheats: &mut HashMap<[Coord; 2], u32>,
        visited: &mut [bool],
        base_score: u32,
        cheat_duration: u8,
    ) {
        if self.idx(&cur.position) == self.end {
            match cur.cheat {
                Cheat::Used(x) => {
                    cheats.insert(x, base_score - cur.cost);
                }
                _ => (),
            }
            return;
        }
        #[cfg(not(test))]
        const N: u32 = 100;
        #[cfg(test)]
        const N: u32 = 50;
        if matches!(cur.cheat, Cheat::Usable) {
            for (position, nei_idx) in self.cheat_neighbors(cheat_duration, cur.position) {
                let cost = cur.cost + position.dist(cur.position);
                if cost <= base_score - N && (!visited[nei_idx]) {
                    let node = MazeNode2 {
                        cost,
                        position,
                        cheat: Cheat::Used([cur.position, position]),
                    };
                    visited[nei_idx] = true;
                    self.dfs_cheat(node, cheats, visited, base_score, cheat_duration);
                    visited[nei_idx] = false;
                }
            }
        }
        for next_dir in DIRECTIONS {
            let position = cur.position + next_dir;
            let cost = cur.cost + 1;
            let nei_idx = self.idx(&position);
            if cost <= base_score - N && self.grid[nei_idx] != b'#' && (!visited[nei_idx]) {
                let node = MazeNode2 {
                    cost,
                    position,
                    cheat: cur.cheat,
                };
                visited[nei_idx] = true;
                self.dfs_cheat(node, cheats, visited, base_score, cheat_duration);
                visited[nei_idx] = false;
            }
        }
    }
    fn dijsktra(&mut self) -> u32 {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct MazeNode {
            cost: u32,
            position: Coord,
        }
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.grid.len()];
        visited[self.start] = true;
        queue.push_back(MazeNode {
            cost: 0,
            position: self.pos(self.start),
        });
        let mut score = u32::MAX;
        while let Some(cur) = queue.pop_front() {
            let cur_idx = self.idx(&cur.position);
            if cur_idx == self.end {
                score = cur.cost;
                break;
            }
            for next_dir in DIRECTIONS {
                let nei = cur.position + next_dir;
                let nei_cost = cur.cost + 1;
                let nei_idx = self.idx(&nei);
                if self.grid[nei_idx] != b'#' && (!visited[nei_idx]) {
                    visited[nei_idx] = true;
                    queue.push_back(MazeNode {
                        cost: nei_cost,
                        position: nei,
                    });
                }
            }
        }
        score
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct MazeNode2 {
    cost: u32,
    position: Coord,
    cheat: Cheat,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cheat {
    Usable,
    Used([Coord; 2]),
}

#[cfg(test)]
const EXAMPLE: &'static str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[test]
fn test1() {
    // only one cheat save 50 picosecs or more
    assert_eq!(part1(EXAMPLE), 1);
}
#[test]
fn test2() {
    let total = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;
    assert_eq!(part2(EXAMPLE), total);
}
