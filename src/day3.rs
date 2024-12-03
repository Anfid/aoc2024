use crate::parsers::u64_from_ascii;
use anyhow::Result;
use aoc_runner_derive::aoc;

#[aoc(day3, part1, AoCS)]
pub fn part1(input: &str) -> u64 {
    part1_safe(input).unwrap()
}

#[aoc(day3, part2, AoCS)]
pub fn part2(input: &str) -> u64 {
    part2_safe(input).unwrap()
}

#[aoc(day3, part1, default)]
pub fn part1_safe(input: &str) -> Result<u64> {
    let mut input = input;
    let mut res = 0;
    while input.len() >= 8 {
        parse_expr(&input).map(|v| res += v);
        input = &input[1..]
    }
    Ok(res)
}

pub fn parse_expr(expr: &str) -> Option<u64> {
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
        match parse_fn(&input) {
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
    fn test_part1() {
        assert_eq!(part1(DAY3_INPUT), 163931492);
        assert_eq!(part1_safe(DAY3_INPUT).unwrap(), 163931492);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DAY3_INPUT), 76911921);
        assert_eq!(part2_safe(DAY3_INPUT).unwrap(), 76911921);
    }
}
