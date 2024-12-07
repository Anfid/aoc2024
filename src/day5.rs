use crate::parsers::num_from_digits;
use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes();

    let (ordering, idx) = parse(input);
    input = &input[idx + 1..];

    let mut res = 0;

    let mut buf = Vec::with_capacity(50);
    loop {
        buf.clear();
        let mut i = 0;
        loop {
            let [d1, d2] = input[i..i + 2] else {
                unreachable!()
            };
            buf.push(num_from_digits!(u32, d1, d2));
            if input.get(i + 2) != Some(&b',') {
                break;
            }
            i += 3;
        }

        let mut correct = true;
        for (idx, n) in buf.iter().enumerate() {
            for (left, right) in &ordering {
                if n == right {
                    if buf[idx..].iter().any(|v| v == left) {
                        correct = false;
                    }
                }
            }
        }
        if correct {
            res += buf[buf.len() / 2];
        }

        if let Some(next) = &input.get(i + 3..) {
            input = next
        } else {
            break;
        }
    }

    res
}

#[aoc(day5, part2)]
pub fn part2(_input: &str) -> u32 {
    todo!()
}

fn parse(input: &[u8]) -> (Vec<(u32, u32)>, usize) {
    let mut pairs = Vec::with_capacity(2000);

    let mut i = 0;
    while let [l1, l2, b'|', r1, r2] = input[i..i + 5] {
        let l = num_from_digits!(u32, l1, l2);
        let r = num_from_digits!(u32, r1, r2);
        pairs.push((l, r));
        i += 6;
    }

    (pairs, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY5_INPUT: &'static str = include_str!("../input/2024/day5.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(DAY5_INPUT), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(DAY5_INPUT), 0);
    }
}
