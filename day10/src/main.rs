use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> usize {
    let mut cache = HashMap::new();
    let mut visited = HashSet::new();
    let mut fifo = VecDeque::new();
    part_x(input, |g, coord| {
        g.score(coord, &mut cache, &mut visited, &mut fifo)
    })
}
fn part2(input: &'static str) -> usize {
    let mut fifo = VecDeque::new();
    part_x(input, |g, coord| g.score2(coord, &mut fifo))
}
fn part_x<F: FnMut(&Grid, Coord) -> usize>(input: &'static str, mut score_func: F) -> usize {
    let g = Grid::new(input);
    (0..g.n)
        .flat_map(|y| (0..g.n).map(move |x| Coord([x, y])))
        .filter_map(|c| {
            let height = g.get(c).unwrap();
            (height == 0).then(|| score_func(&g, c))
        })
        .sum()
}
struct Grid {
    n: isize,
    buf: Vec<u8>,
}
impl Grid {
    fn score2(&self, start: Coord, queue: &mut VecDeque<Coord>) -> usize {
        queue.push_front(start);
        let mut count = 0;
        while let Some(coord) = queue.pop_front() {
            let height = self.get(coord).unwrap();
            if height == 9 {
                count += 1;
            }
            for (nei_coord, nei_height) in self.get_neighbors(coord) {
                if nei_height.checked_sub(height).is_some_and(|dh| dh == 1) {
                    queue.push_back(nei_coord);
                }
            }
        }
        count
    }
    fn score(
        &self,
        start: Coord,
        cache: &mut HashMap<Coord, usize>,
        visited: &mut HashSet<Coord>,
        queue: &mut VecDeque<Coord>,
    ) -> usize {
        if let Some(&v) = cache.get(&start) {
            return v;
        }
        queue.push_front(start);
        visited.insert(start);
        let mut count = 0;
        while let Some(coord) = queue.pop_front() {
            let height = self.get(coord).unwrap();
            if height == 9 {
                count += 1;
            }
            for (nei_coord, nei_height) in self.get_neighbors(coord) {
                if nei_height.checked_sub(height).is_some_and(|dh| dh == 1)
                    && (!visited.contains(&nei_coord))
                {
                    queue.push_back(nei_coord);
                    visited.insert(nei_coord);
                }
            }
        }
        let to_cache = visited
            .iter()
            .filter_map(|&x| self.get(x).is_some_and(|h| h == 0).then_some((x, count)));
        cache.extend(to_cache);
        visited.clear();
        count
    }
    fn new(input: &'static str) -> Self {
        let input = input.trim();
        let n = input.lines().count() as isize;
        let buf = input
            .lines()
            .flat_map(|s| s.as_bytes().iter().map(|&c| c - b'0'))
            .collect::<Vec<u8>>();
        Grid { n, buf }
    }
    fn get_neighbors(&self, coord: Coord) -> impl Iterator<Item = (Coord, u8)> + '_ {
        [Coord([1, 0]), Coord([-1, 0]), Coord([0, 1]), Coord([0, -1])]
            .into_iter()
            .filter_map(move |dir| {
                let nei = coord + dir;
                self.get(nei).map(|h| (nei, h))
            })
    }
    fn get(&self, coord: Coord) -> Option<u8> {
        let [x, y] = coord.0;
        if (0..self.n).contains(&x) && (0..self.n).contains(&y) {
            Some(self.buf[(x + self.n * y) as usize])
        } else {
            None
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([isize; 2]);

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

#[cfg(test)]
const EXAMPLE: &'static str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 36);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 81);
}
