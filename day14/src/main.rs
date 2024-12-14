use std::ops::{Add, Div, Mul, Rem};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> usize {
    let size_mid = SIZE / 2;
    let mut quadrants = [0, 0, 0, 0];
    let robots = robots_n_steps(input, 100);
    for [next_pos, _] in robots {
        if next_pos.0[0] == size_mid.0[0] || next_pos.0[1] == size_mid.0[1] {
            continue;
        }
        let quad = next_pos / (size_mid + 1);
        let i = quad.0[0] + 2 * quad.0[1];
        quadrants[i as usize] += 1;
    }
    quadrants.into_iter().product()
}
fn part2(input: &'static str) -> usize {
    const X: usize = SIZE.0[0] as usize;
    const Y: usize = SIZE.0[1] as usize;
    const START_X: usize = X * 2 / 5;
    const START_Y: usize = Y * 2 / 5;
    const END_X: usize = X * 3 / 5;
    const END_Y: usize = Y * 3 / 5;
    const N: usize = 5;
    let mut robots = robots_n_steps(input, 0);
    for i in 0..10_000 {
        let mut grid = [[' '; X]; Y];
        for [pos, _] in robots.iter() {
            let [x, y] = pos.0.map(|x| x as usize);
            grid[y][x] = 'x';
        }
        if (START_Y..END_Y)
            .flat_map(|y| (START_X..END_X).map(move |x| (x, y)))
            .any(|(x, y)| {
                (0..N)
                    .flat_map(|dy| (0..N).map(move |dx| (dx, dy)))
                    .all(|(dx, dy)| grid[y + dy][x + dx] == 'x')
            })
        {
            println!(
                "{}",
                grid.into_iter()
                    .map(|arr| String::from_iter(arr.iter()))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            return i as usize;
        }
        robots.iter_mut().for_each(|[pos, vel]| {
            *pos = (((*pos + *vel) % SIZE) + SIZE) % SIZE;
        })
    }
    usize::default()
}

fn robots_n_steps(input: &'static str, n: isize) -> Vec<[Coord; 2]> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split_ascii_whitespace().map(|s| Coord::new(&s[2..]));
            let pos = it.next().unwrap();
            let vel = it.next().unwrap();
            let mut next_pos = (pos + (vel * n)) % SIZE;
            if next_pos.0[0] < 0 || next_pos.0[1] < 0 {
                next_pos = (next_pos + SIZE) % SIZE;
            }
            [next_pos, vel]
        })
        .collect()
}

#[cfg(not(test))]
const SIZE: Coord = Coord([101, 103]);
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord([isize; 2]);

impl Coord {
    fn new(s: &'static str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Coord([x.parse().unwrap(), y.parse().unwrap()])
    }
}
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Add<isize> for Coord {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Coord([self.0[0] + rhs, self.0[1] + rhs])
    }
}
impl Mul<isize> for Coord {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Coord([self.0[0] * rhs, self.0[1] * rhs])
    }
}
impl Div<isize> for Coord {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Coord([self.0[0] / rhs, self.0[1] / rhs])
    }
}
impl Div for Coord {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] / rhs.0[0], self.0[1] / rhs.0[1]])
    }
}
impl Rem for Coord {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Coord([self.0[0] % rhs.0[0], self.0[1] % rhs.0[1]])
    }
}
#[cfg(test)]
const SIZE: Coord = Coord([11, 7]);
#[cfg(test)]
const EXAMPLE: &'static str = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 12);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 6);
}
