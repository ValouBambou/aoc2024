use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Mul},
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1::<100>(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2::<100>(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1<const N: u32>(input: &'static str) -> u32 {
    part_x::<N>(input, 2)
}
fn part2<const N: u32>(input: &'static str) -> u32 {
    part_x::<N>(input, 20)
}
fn part_x<const N: u32>(input: &'static str, cheat_duration: i16) -> u32 {
    let g = Grid::new(input);
    let base_cost = g.dijsktra();
    g.dijsktra_cheat::<N>(base_cost, cheat_duration)
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
    fn cheat_neighbors(
        &self,
        cheat_duration: i16,
        cur: Coord,
    ) -> impl Iterator<Item = (Coord, usize)> + '_ {
        ((-cheat_duration)..=cheat_duration).flat_map(move |ix| {
            ((-cheat_duration)..=cheat_duration).filter_map(move |iy| {
                let dist = ix.abs() + iy.abs();
                let nei = cur + Coord([ix, iy]);
                if !((1..=cheat_duration).contains(&dist)
                    && (0..self.n).contains(&nei.0[0])
                    && (0..self.n).contains(&nei.0[1]))
                {
                    return None;
                }
                let idx = self.idx(&nei);
                (self.grid[idx] != b'#').then_some((nei, idx))
            })
        })
    }
    fn dijsktra(&self) -> Vec<usize> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.grid.len()];
        let mut prev = vec![usize::MAX; self.grid.len()];
        visited[self.start] = true;
        queue.push_back((0, self.pos(self.start)));
        while let Some((cur_cost, cur_pos)) = queue.pop_front() {
            let cur_idx = self.idx(&cur_pos);
            if cur_idx == self.end {
                break;
            }
            for next_dir in DIRECTIONS {
                let nei = cur_pos + next_dir;
                let nei_cost = cur_cost + 1;
                let nei_idx = self.idx(&nei);
                if self.grid[nei_idx] != b'#' && (!visited[nei_idx]) {
                    visited[nei_idx] = true;
                    prev[nei_idx] = cur_idx;
                    queue.push_back((nei_cost, nei));
                }
            }
        }
        let mut cur = self.end;
        let mut path = Vec::new();
        while self.start != cur {
            path.push(cur);
            cur = prev[cur];
        }
        path.push(self.start);
        path.reverse();
        path
    }
    fn dijsktra_cheat<const N: u32>(&self, base_path: Vec<usize>, cheat_duration: i16) -> u32 {
        let mut cheats = HashSet::new();
        let mut table = vec![None; self.grid.len()];
        for (i, &c) in base_path.iter().enumerate() {
            table[c] = Some(i);
        }
        for (i, cur) in base_path.into_iter().enumerate() {
            let cur_pos = self.pos(cur);
            for (nei_pos, nei) in self.cheat_neighbors(cheat_duration, cur_pos) {
                let dist = nei_pos.dist(cur_pos);
                let i2 = table[nei];
                if i2.is_some_and(|i2| i2.saturating_sub(i) >= (dist + N) as usize) {
                    cheats.insert([cur_pos, nei_pos]);
                }
            }
        }
        cheats.len() as u32
    }
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
    let total = 14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1;
    assert_eq!(part1::<2>(EXAMPLE), total);
}
#[test]
fn test2() {
    let total = 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3;
    assert_eq!(part2::<50>(EXAMPLE), total);
}
