use anyhow::Result;
use aoc_runner_derive::aoc;
use rayon::prelude::*;

#[aoc(day7, part1, AoCS)]
pub fn part1(input: &str) -> u64 {
    part1_safe(input).unwrap()
}

#[aoc(day7, part1, default)]
pub fn part1_safe(input: &str) -> Result<u64> {
    let values = input.par_lines().map(|line| {
        let Some((res, values)) = line.split_once(':') else {
            anyhow::bail!("Invalid input format")
        };
        let expected: u64 = res.parse()?;
        let values = values
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        for i in 0..1 << (values.len() - 1) {
            let op_res =
                values[1..]
                    .iter()
                    .copied()
                    .enumerate()
                    .fold(values[0], |acc, (idx, x)| {
                        if i & (1 << idx) != 0 {
                            acc * x
                        } else {
                            acc + x
                        }
                    });
            if expected == op_res {
                return Ok(expected);
            }
        }
        Ok(0)
    });
    values.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY7_INPUT: &'static str = include_str!("../input/2024/day7.txt");
    const DAY7_EXAMPLE: &'static str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1_safe(DAY7_EXAMPLE).unwrap(), 3749);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1_safe(DAY7_INPUT).unwrap(), 7885693428401);
    }
}
