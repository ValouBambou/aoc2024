use std::iter::repeat;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

const EMPTY: usize = usize::MAX;
fn part1(input: &'static str) -> usize {
    let compact1 = |files: &mut Vec<usize>| {
        let mut start = 0;
        while start < files.len() {
            while *files.last().unwrap() == EMPTY {
                files.pop().unwrap();
            }
            if files[start] == EMPTY {
                files[start] = files.pop().unwrap();
            }
            start += 1;
        }
    };
    part_x(input, compact1)
}

fn part2(input: &'static str) -> usize {
    let compact2 = |files: &mut Vec<usize>| {
        let mut end = files.len();
        let mut current_file = files[end - 1];
        let mut start = 0;
        while current_file > files[0] {
            while files[end - 1] != current_file {
                end -= 1;
            }
            while files[start] != EMPTY {
                start += 1;
            }
            let len_last = files
                .iter()
                .rev()
                .skip(files.len() - end)
                .take_while(|&&x| x == current_file)
                .count();
            if let Some(start_i) = (start..(end - len_last))
                .find(|&i| files[i..(i + len_last)].iter().all(|&id| id == EMPTY))
            {
                let (left, right) = files.split_at_mut(start_i + len_last);
                let end2 = end - start_i - len_last;
                left[start_i..(start_i + len_last)]
                    .copy_from_slice(&right[(end2 - len_last)..end2]);
                files[(end - len_last)..end].fill(EMPTY);
                if start_i == start {
                    start += len_last;
                }
            }
            end -= len_last;
            current_file -= 1;
        }
    };
    part_x(input, compact2)
}

fn part_x(input: &'static str, mut compact: impl FnMut(&mut Vec<usize>)) -> usize {
    let mut files = input
        .trim()
        .as_bytes()
        .iter()
        .enumerate()
        .flat_map(|(i, &c)| {
            repeat(if i % 2 == 0 { i / 2 } else { EMPTY }).take((c - b'0') as usize)
        })
        .collect::<Vec<usize>>();
    compact(&mut files);
    files
        .into_iter()
        .enumerate()
        .map(|(pos, id)| if id != EMPTY { pos * id } else { 0 })
        .sum()
}

#[cfg(test)]
const EXAMPLE: &'static str = "2333133121414131402";
#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 1928);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 2858);
}
