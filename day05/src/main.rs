const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> u32 {
    let (constraints, lines) = parse(input);
    lines
        .filter_map(|v| {
            verify(&v, &constraints)
                .is_none()
                .then(|| v[v.len() / 2] as u32)
        })
        .sum()
}
fn part2(input: &'static str) -> u32 {
    let (constraints, lines) = parse(input);
    lines
        .filter_map(|mut v| {
            // TODO: retry the idea of DAG and reacheable nodes as comparison fn
            // it should use more memory but less time
            // since this buble sort is O(constraints.len * v.len * nshuffle)
            let mut sorted = false;
            while let Some((i1, i2)) = verify(&v, &constraints) {
                let tmp = v[i1];
                v[i1] = v[i2];
                v[i2] = tmp;
                sorted = true;
            }
            sorted.then_some(v[v.len() / 2] as u32)
        })
        .sum()
}

fn parse(input: &'static str) -> (Vec<(u8, u8)>, impl Iterator<Item = Vec<u8>>) {
    let paragraphs = input.trim().split_once("\n\n").unwrap();
    (
        paragraphs
            .0
            .lines()
            .map(|l| {
                let (n1, n2) = l.split_once("|").unwrap();
                let n1: u8 = n1.parse().unwrap();
                let n2: u8 = n2.parse().unwrap();
                (n1, n2)
            })
            .collect::<Vec<(u8, u8)>>(),
        paragraphs.1.lines().map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u8>>()
        }),
    )
}

fn verify(nums: &[u8], dummy: &[(u8, u8)]) -> Option<(usize, usize)> {
    dummy.iter().find_map(|(n1, n2)| {
        let i1 = nums.iter().position(|x| x == n1);
        let i2 = nums.iter().position(|x| x == n2);
        if let (Some(i1), Some(i2)) = (i1, i2) {
            (i1 >= i2).then_some((i1, i2))
        } else {
            None
        }
    })
}

#[cfg(test)]
const EXAMPLE: &'static str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 143);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 123);
}
