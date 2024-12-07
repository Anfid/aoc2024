use crate::parsers::{num_from_digits, u64_from_ascii};
use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use std::cmp::Ordering;

#[aoc(day1, part1, AoCS)]
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let (mut left, mut right) = parse_optimistic(input);
    left.sort_unstable();
    right.sort_unstable();
    std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2, AoCS)]
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let parsed = parse_optimistic(input);
    solve_p2(parsed)
}

pub fn parse_optimistic(input: &[u8]) -> (Vec<u64>, Vec<u64>) {
    const WIDTH: usize = 14;
    const HEIGHT: usize = 1000;
    let mut left = Vec::with_capacity(HEIGHT);
    let mut right = Vec::with_capacity(HEIGHT);

    for i in 0..HEIGHT {
        let [l1, l2, l3, l4, l5, b' ', b' ', b' ', r1, r2, r3, r4, r5] =
            input[i * WIDTH..i * WIDTH + WIDTH - 1]
        else {
            unreachable!()
        };
        let l = num_from_digits!(u64, l1, l2, l3, l4, l5);
        let r = num_from_digits!(u64, r1, r2, r3, r4, r5);
        left.push(l);
        right.push(r);
    }
    (left, right)
}

#[aoc(day1, part1, default)]
pub fn part1_safe(input: &str) -> Result<u64> {
    let (mut left, mut right) = parse(input)?;
    left.sort_unstable();
    right.sort_unstable();
    let result = std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    Ok(result)
}

#[aoc(day1, part2, default)]
pub fn part2_safe(input: &str) -> Result<u64> {
    let parsed = parse(input)?;
    Ok(solve_p2(parsed))
}

pub fn solve_p2((mut left, mut right): (Vec<u64>, Vec<u64>)) -> u64 {
    left.sort_unstable();
    right.sort_unstable();

    let mut ileft = left.into_iter();
    let mut iright = right.into_iter();
    let mut nleft = ileft.next();
    let mut nright = iright.next();
    let mut similarity_score = 0;
    while let (Some(l), Some(r)) = (nleft, nright) {
        match l.cmp(&r) {
            Ordering::Less => {
                nleft = ileft.next();
            }
            Ordering::Equal => {
                let v = l;
                let mut lcount = 0;
                while nleft == Some(v) {
                    nleft = ileft.next();
                    lcount += 1;
                }
                let mut rcount = 0;
                while nright == Some(v) {
                    nright = iright.next();
                    rcount += 1;
                }
                similarity_score += v * lcount * rcount
            }
            Ordering::Greater => {
                nright = iright.next();
            }
        }
    }
    similarity_score
}

pub fn parse(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    let bytes = input.as_bytes();
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    for line in bytes.split_inclusive(|&c| c == b'\n') {
        let line = line.trim_ascii_start();
        let space = line
            .iter()
            .position(|&c| c == b' ')
            .ok_or_else(|| anyhow!("not enough location ids"))?;
        let (l, r) = line.split_at(space);
        let l = u64_from_ascii(l);
        let r = u64_from_ascii(r);
        left.push(l);
        right.push(r);
    }
    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY1_INPUT: &'static str = include_str!("../input/2024/day1.txt");
    const DAY1_EXAMPLE: &'static str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn part1_example() {
        assert_eq!(part1_safe(DAY1_EXAMPLE).unwrap(), 11);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(DAY1_INPUT), 1941353);
        assert_eq!(part1_safe(DAY1_INPUT).unwrap(), 1941353);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_safe(DAY1_EXAMPLE).unwrap(), 31);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(DAY1_INPUT), 22539317);
        assert_eq!(part2_safe(DAY1_INPUT).unwrap(), 22539317);
    }
}
