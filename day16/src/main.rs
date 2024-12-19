use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, Sub},
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> u32 {
    let g = Grid::new(input);
    g.dijsktra().0
}
fn part2(input: &'static str) -> usize {
    let g = Grid::new(input);
    let prev = g.dijsktra().1;
    let end = g.idx(&g.end);
    let mut path = prev
        .iter()
        .filter_map(|(k, v)| (k.0 == end).then_some(v.iter()))
        .flatten()
        .cloned()
        .collect::<Vec<(usize, Coord)>>();
    let mut visited = path.iter().cloned().collect::<HashSet<(usize, Coord)>>();
    visited.insert((end, Coord([-1, 0])));
    visited.insert((end, Coord([1, 0])));
    visited.insert((end, Coord([0, 1])));
    visited.insert((end, Coord([0, -1])));
    let empty = Vec::new();
    while let Some(pair) = path.pop() {
        for pair2 in prev.get(&pair).unwrap_or(&empty) {
            if !visited.contains(&pair2) {
                visited.insert(*pair2);
                path.push(*pair2);
            }
        }
    }
    visited
        .into_iter()
        .map(|x| x.0)
        .collect::<HashSet<usize>>()
        .len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([i32; 2]);
impl Coord {
    fn rotate_right(&self) -> Self {
        Coord(match self.0 {
            [0, y] => [-y, 0],
            [x, 0] => [0, x],
            c => panic!("rotate {:?} not supported", c),
        })
    }
    fn rotate_left(&self) -> Self {
        Coord(match self.0 {
            [0, y] => [y, 0],
            [x, 0] => [0, -x],
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
    n: i32,
    grid: Vec<u8>,
    start: Coord,
    end: Coord,
    direction: Coord,
}

impl Grid {
    fn new(input: &'static str) -> Self {
        let input = input.trim();
        let n = input.lines().count();
        let grid = input
            .lines()
            .flat_map(|s| s.as_bytes().iter().cloned())
            .collect::<Vec<u8>>();
        let start = grid.iter().position(|&x| x == b'S').unwrap() as i32;
        let end = grid.iter().position(|&x| x == b'E').unwrap() as i32;
        let n = n as i32;
        let start = Coord([start % n, start / n]);
        let end = Coord([end % n, end / n]);
        let direction = Coord([1, 0]);
        Grid {
            n,
            grid,
            start,
            end,
            direction,
        }
    }
    #[inline(always)]
    fn idx(&self, coord: &Coord) -> usize {
        let [x, y] = coord.0;
        debug_assert!((0..self.n).contains(&x));
        debug_assert!((0..self.n).contains(&y));

        (x + self.n * y) as usize
    }
    fn dijsktra(&self) -> (u32, HashMap<(usize, Coord), Vec<(usize, Coord)>>) {
        let mut priority_queue = BinaryHeap::new();
        // let mut dist = self.grid.iter().map(|_| u32::MAX).collect::<Vec<u32>>();
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();
        let start = self.idx(&self.start);
        dist.insert((start, self.direction), 0);
        let end = self.idx(&self.end);
        priority_queue.push(MazeNode {
            cost: 0,
            position: self.start,
            direction: self.direction,
        });
        let mut best_cost = u32::MAX;
        while let Some(cur) = priority_queue.pop() {
            let cur_idx = self.idx(&cur.position);
            if cur_idx == end {
                best_cost = best_cost.min(cur.cost);
            }
            let nexts = [
                (cur.direction, 1),
                (cur.direction.rotate_right(), 1001),
                (cur.direction.rotate_left(), 1001),
            ];
            for (next_dir, cost) in nexts {
                let nei = cur.position + next_dir;
                let nei_idx = self.idx(&nei);
                let nei_cost = cur.cost + cost;
                let nk = (nei_idx, next_dir);
                let known_cost = dist.get(&nk).cloned().unwrap_or(u32::MAX);
                if self.grid[nei_idx] != b'#' && nei_cost <= known_cost && nei_cost <= best_cost {
                    dist.insert(nk, nei_cost);
                    if nei_cost < known_cost {
                        prev.entry(nk).or_insert(Vec::new()).clear();
                    }
                    prev.get_mut(&nk).unwrap().push((cur_idx, cur.direction));
                    priority_queue.push(MazeNode {
                        cost: nei_cost,
                        position: nei,
                        direction: next_dir,
                    });
                }
            }
        }
        (best_cost, prev)
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct MazeNode {
    cost: u32,
    position: Coord,
    direction: Coord,
}
impl Ord for MazeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for MazeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(test)]
const EXAMPLE1: &'static str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
#[cfg(test)]
const EXAMPLE2: &'static str = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE1), 7036);
    assert_eq!(part1(EXAMPLE2), 11048);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE1), 45);
    assert_eq!(part2(EXAMPLE2), 64);
}
