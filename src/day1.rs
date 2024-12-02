use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use std::cmp::Ordering;

#[aoc(day1, part1, AoCS)]
pub fn part1(input: &str) -> i64 {
    part1_safe(input).unwrap()
}

#[aoc(day1, part2, AoCS)]
pub fn part2(input: &str) -> i64 {
    part2_safe(input).unwrap()
}

#[aoc(day1, part1, default)]
pub fn part1_safe(input: &str) -> Result<i64> {
    let (mut left, mut right) = parse(input)?;
    left.sort_unstable();
    right.sort_unstable();
    let result = std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r) as i64)
        .sum();
    Ok(result)
}

#[aoc(day1, part2, default)]
pub fn part2_safe(input: &str) -> Result<i64> {
    let (mut left, mut right) = parse(input)?;
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
    Ok(similarity_score)
}

pub fn parse(input: &str) -> Result<(Vec<i64>, Vec<i64>)> {
    let bytes = input.as_bytes();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in bytes.split_inclusive(|&c| c == b'\n') {
        let line = line.trim_ascii_start();
        let space = line
            .iter()
            .position(|&c| c == b' ')
            .ok_or_else(|| anyhow!("not enough location ids"))?;
        let (l, r) = line.split_at(space);
        let l = i64_from_ascii(l.trim_ascii_end());
        let r = i64_from_ascii(r.trim_ascii());
        left.push(l);
        right.push(r);
    }
    Ok((left, right))
}

fn i64_from_ascii(bytes: &[u8]) -> i64 {
    bytes
        .into_iter()
        .fold(0i64, |acc, &d| acc * 10 + (d - b'0') as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY1_INPUT: &'static str = "\
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DAY1_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DAY1_INPUT), 31);
    }
}
