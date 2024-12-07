use crate::parsers::{num_from_digits, u64_from_ascii};
use anyhow::Result;
use aoc_runner_derive::aoc;

#[aoc(day3, part1, AoCS)]
pub fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut res = 0;
    loop {
        let Some(pos) = input.iter().position(|&c| c == b'm') else {
            break;
        };
        let (offset, v) = parse_ascii_mul(&input[pos..]);
        if let Some(v) = v {
            res += v;
        }
        input = &input[offset + pos..];
    }
    res
}

fn parse_ascii_mul(expr: &[u8]) -> (usize, Option<u64>) {
    if expr.starts_with(b"mul(") {
        parse_mul_args(expr)
    } else {
        (1, None)
    }
}

/// Parse mul function arguments with explicit pattern matching for every possible case
#[inline(always)]
fn parse_mul_args(args: &[u8]) -> (usize, Option<u64>) {
    match args[4..] {
        [l1, b',', r1, b')', ..] => (
            8,
            Some(num_from_digits!(u64, l1) * num_from_digits!(u64, r1)),
        ),
        [l1, b',', r1, r2, b')', ..] => (
            9,
            Some(num_from_digits!(u64, l1) * num_from_digits!(u64, r1, r2)),
        ),
        [l1, b',', r1, r2, r3, b')', ..] => (
            10,
            Some(num_from_digits!(u64, l1) * num_from_digits!(u64, r1, r2, r3)),
        ),
        [l1, l2, b',', r1, b')', ..] => (
            9,
            Some(num_from_digits!(u64, l1, l2) * num_from_digits!(u64, r1)),
        ),
        [l1, l2, b',', r1, r2, b')', ..] => (
            10,
            Some(num_from_digits!(u64, l1, l2) * num_from_digits!(u64, r1, r2)),
        ),
        [l1, l2, b',', r1, r2, r3, b')', ..] => (
            11,
            Some(num_from_digits!(u64, l1, l2) * num_from_digits!(u64, r1, r2, r3)),
        ),
        [l1, l2, l3, b',', r1, b')', ..] => (
            10,
            Some(num_from_digits!(u64, l1, l2, l3) * num_from_digits!(u64, r1)),
        ),
        [l1, l2, l3, b',', r1, r2, b')', ..] => (
            11,
            Some(num_from_digits!(u64, l1, l2, l3) * num_from_digits!(u64, r1, r2)),
        ),
        [l1, l2, l3, b',', r1, r2, r3, b')', ..] => (
            12,
            Some(num_from_digits!(u64, l1, l2, l3) * num_from_digits!(u64, r1, r2, r3)),
        ),
        _ => (4, None),
    }
}

#[aoc(day3, part2, AoCS)]
pub fn part2(input: &str) -> u64 {
    let mut input = input;
    let mut res = 0;
    let mut enabled = true;
    loop {
        if enabled {
            let Some(pos) = input.bytes().position(|c| c == b'm' || c == b'd') else {
                break;
            };
            let (offset, enable, v) = parse_enabled(&input.as_bytes()[pos..]);
            enabled = enable;
            if let Some(v) = v {
                res += v;
            }
            input = &input[offset + pos..];
        } else {
            let Some(pos) = input.find("do()") else {
                break;
            };
            enabled = true;
            input = &input[pos + 4..];
        }
    }
    res
}

fn parse_enabled(expr: &[u8]) -> (usize, bool, Option<u64>) {
    if expr.starts_with(b"mul(") {
        let (offset, v) = parse_mul_args(expr);
        (offset, true, v)
    } else if expr.starts_with(b"don't()") {
        (7, false, None)
    } else {
        (1, true, None)
    }
}

#[aoc(day3, part1, default)]
pub fn part1_safe(input: &str) -> Result<u64> {
    let mut input = input;
    let mut res = 0;
    while input.len() >= 8 {
        if let Some(v) = parse_mul(input) {
            res += v;
        }
        input = &input[1..]
    }
    Ok(res)
}

pub fn parse_mul(expr: &str) -> Option<u64> {
    if expr.starts_with("mul(") {
        let args = &expr[4..12];
        let sep = args.find(',')?;
        let l = &args[..sep];
        let cp = args[sep + 1..].find(')')?;
        let r = &args[sep + 1..sep + 1 + cp];
        assert!(l.bytes().all(|b| b.is_ascii_digit()));
        assert!(r.bytes().all(|b| b.is_ascii_digit()));

        Some(u64_from_ascii(l.as_bytes()) * u64_from_ascii(r.as_bytes()))
    } else {
        None
    }
}

#[aoc(day3, part2, default)]
pub fn part2_safe(input: &str) -> Result<u64> {
    let mut input = input;
    let mut res = 0;
    let mut enabled = true;
    while input.len() >= 8 {
        match parse_fn(input) {
            Some(Expr::Stop) => {
                enabled = false;
            }
            Some(Expr::Start) => {
                enabled = true;
            }
            Some(Expr::Value(v)) if enabled => {
                res += v;
            }
            _ => {}
        }
        input = &input[1..]
    }
    Ok(res)
}

enum Expr {
    Stop,
    Start,
    Value(u64),
}

fn parse_fn(expr: &str) -> Option<Expr> {
    if expr.starts_with("mul(") {
        let args = &expr[4..12];
        let sep = args.find(',')?;
        let l = &args[..sep];
        let cp = args[sep + 1..].find(')')?;
        let r = &args[sep + 1..sep + 1 + cp];
        assert!(l.bytes().all(|b| b.is_ascii_digit()));
        assert!(r.bytes().all(|b| b.is_ascii_digit()));

        Some(Expr::Value(
            u64_from_ascii(l.as_bytes()) * u64_from_ascii(r.as_bytes()),
        ))
    } else if expr.starts_with("do()") {
        Some(Expr::Start)
    } else if expr.starts_with("don't()") {
        Some(Expr::Stop)
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const DAY3_INPUT: &'static str = include_str!("../input/2024/day3.txt");

    #[test]
    fn part1_input() {
        assert_eq!(part1(DAY3_INPUT), 163931492);
        assert_eq!(part1_safe(DAY3_INPUT).unwrap(), 163931492);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(DAY3_INPUT), 76911921);
        assert_eq!(part2_safe(DAY3_INPUT).unwrap(), 76911921);
    }
}
