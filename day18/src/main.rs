use std::{collections::VecDeque, ops::Add};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

#[cfg(test)]
const N: i8 = 7;
#[cfg(not(test))]
const N: i8 = 71;
#[cfg(test)]
const N_FIRST: usize = 12;
#[cfg(not(test))]
const N_FIRST: usize = 1024;

fn part1(input: &'static str) -> u32 {
    let mut g = Grid::new(N);
    for line in input.trim().lines().take(N_FIRST) {
        let (x, y) = line.split_once(',').unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        let i = x + y * (N as usize);
        g.grid[i] = b'#';
    }
    g.dijsktra()
}
fn part2(input: &'static str) -> &'static str {
    let walls = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            x + y * (N as usize)
        })
        .collect::<Vec<usize>>();
    let mut g = Grid::new(N);
    let mut min = N_FIRST;
    let mut max = walls.len() - 1;
    let mut mid = (min + max) / 2;
    let mut pair = [
        g.try_dijkstra(&walls[..mid]),
        g.try_dijkstra(&walls[..(mid + 1)]),
    ];
    while pair != [true, false] {
        match pair {
            [true, true] => {
                min = mid;
                mid = (mid + max) / 2;
            }
            [false, false] => {
                max = mid;
                mid = (min + mid) / 2;
            }
            _ => panic!("wtf"),
        }
        pair = [
            g.try_dijkstra(&walls[..mid]),
            g.try_dijkstra(&walls[..(mid + 1)]),
        ];
    }
    input.trim().lines().nth(mid).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([i8; 2]);
const DIRECTIONS: [Coord; 4] = [Coord([1, 0]), Coord([0, 1]), Coord([-1, 0]), Coord([0, -1])];
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Coord {
    fn is_inside(&self) -> bool {
        let [x, y] = self.0;
        (0..N).contains(&x) && (0..N).contains(&y)
    }
}

struct Grid {
    n: i8,
    grid: Vec<u8>,
}
const END: Coord = Coord([N - 1, N - 1]);
impl Grid {
    fn try_dijkstra(&mut self, walls: &[usize]) -> bool {
        for &i in walls {
            self.grid[i] = b'#';
        }
        let res = self.dijsktra();
        for &i in walls {
            self.grid[i] = b'.';
        }
        res != u32::MAX
    }
    fn new(n: i8) -> Self {
        let nu = n as usize;
        let grid = vec![b'.'; nu * nu];
        Grid { n, grid }
    }
    #[inline(always)]
    fn idx(&self, coord: &Coord) -> usize {
        let [x, y] = coord.0;
        x as usize + (self.n as usize) * (y as usize)
    }
    fn dijsktra(&self) -> u32 {
        let mut priority_queue = VecDeque::new();
        let mut dist = self.grid.iter().map(|_| u32::MAX).collect::<Vec<u32>>();
        let start = 0;
        dist[start] = 0;
        priority_queue.push_back(MazeNode {
            cost: 0,
            position: Coord([0, 0]),
        });
        while let Some(cur) = priority_queue.pop_front() {
            for next_dir in DIRECTIONS {
                let nei = cur.position + next_dir;
                let nei_cost = cur.cost + 1;
                if nei == END {
                    return nei_cost;
                }
                if !nei.is_inside() {
                    continue;
                }
                let nei_idx = self.idx(&nei);
                if self.grid[nei_idx] != b'#' && nei_cost < dist[nei_idx] {
                    dist[nei_idx] = nei_cost;
                    priority_queue.push_back(MazeNode {
                        cost: nei_cost,
                        position: nei,
                    });
                }
            }
        }
        u32::MAX
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct MazeNode {
    cost: u32,
    position: Coord,
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
const EXAMPLE: &'static str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 22);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), "6,1");
}
