const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> usize {
    let grid = CharGrid::new(input);
    let [nrows, ncols] = grid.dims();
    (0..nrows)
        .flat_map(|i| (0..ncols).map(move |j| (i, j)))
        .map(|(i, j)| grid.n_xmas1(i, j))
        .sum()
}
fn part2(input: &'static str) -> usize {
    let grid = CharGrid::new(input);
    let [nrows, ncols] = grid.dims();
    (1..(nrows - 1))
        .flat_map(|i| (1..(ncols - 1)).map(move |j| (i, j)))
        .map(|(i, j)| grid.n_xmas2(i, j))
        .sum()
}

struct CharGrid {
    matrix: Vec<&'static [u8]>,
}

impl CharGrid {
    fn new(input: &'static str) -> Self {
        CharGrid {
            matrix: input.trim().lines().map(|s| s.as_bytes()).collect(),
        }
    }
    fn dims(&self) -> [usize; 2] {
        [self.matrix.len(), self.matrix[0].len()]
    }
    fn n_xmas1(&self, i: usize, j: usize) -> usize {
        if self.matrix[i][j] != b'X' {
            return 0;
        }
        let mut total = 0;
        for di in [-1, 0, 1] {
            for dj in [-1, 0, 1] {
                if di == 0 && dj == 0 {
                    continue;
                }
                total += self.xmas1_dir(i, j, di, dj).unwrap_or(0);
            }
        }
        total
    }
    fn n_xmas2(&self, i: usize, j: usize) -> usize {
        if self.matrix[i][j] != b'A' {
            return 0;
        }
        let mut total: usize = 0;
        for di in [-1, 1] {
            for dj in [-1, 1] {
                let i2 = i.wrapping_add_signed(di);
                let j2 = j.wrapping_add_signed(dj);
                let i3 = i.wrapping_add_signed(-di);
                let j3 = j.wrapping_add_signed(-dj);
                if self.matrix[i2][j2] == b'M' && self.matrix[i3][j3] == b'S' {
                    total += 1;
                }
            }
        }
        total.saturating_sub(1)
    }
    fn xmas1_dir(&self, mut i: usize, mut j: usize, di: isize, dj: isize) -> Option<usize> {
        for chr in [b'M', b'A', b'S'] {
            i = i.checked_add_signed(di)?;
            j = j.checked_add_signed(dj)?;
            if &chr != self.matrix.get(i)?.get(j)? {
                return None;
            }
        }
        Some(1)
    }
}

#[cfg(test)]
const EXAMPLE: &'static str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 18);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 9);
}
