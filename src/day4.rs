use anyhow::Result;
use aoc_runner_derive::aoc;

const WIDTH: usize = 141;
const HEIGHT: usize = 140;

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

#[inline(always)]
fn get_idx(row: Option<usize>, col: Option<usize>) -> Option<usize> {
    match (row, col) {
        (Some(row), Some(col)) => Some(row * WIDTH + col),
        _ => None,
    }
}

#[aoc(day4, part1, AoCS)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut res = 0;

    for row in 0..HEIGHT {
        for col in 0..WIDTH - 1 {
            if input[row * WIDTH + col] == X {
                if let Some(&S) =
                    get_idx(Some(row), col.checked_sub(3)).and_then(|idx| input.get(idx))
                {
                    let a = input[row * WIDTH + col - 2];
                    let m = input[row * WIDTH + col - 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                }

                if let Some(&S) = input.get(row * WIDTH + col + 3) {
                    let a = input[row * WIDTH + col + 2];
                    let m = input[row * WIDTH + col + 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                // --------------

                if let Some(&S) =
                    get_idx(row.checked_sub(3), col.checked_sub(3)).and_then(|idx| input.get(idx))
                {
                    let a = input[(row - 2) * WIDTH + col - 2];
                    let m = input[(row - 1) * WIDTH + col - 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                if let Some(&S) =
                    get_idx(row.checked_sub(3), Some(col)).and_then(|idx| input.get(idx))
                {
                    let a = input[(row - 2) * WIDTH + col];
                    let m = input[(row - 1) * WIDTH + col];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                if let Some(&S) =
                    get_idx(row.checked_sub(3), Some(col + 3)).and_then(|idx| input.get(idx))
                {
                    let a = input[(row - 2) * WIDTH + col + 2];
                    let m = input[(row - 1) * WIDTH + col + 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                // --------------

                if let Some(&S) =
                    get_idx(Some(row + 3), col.checked_sub(3)).and_then(|idx| input.get(idx))
                {
                    let a = input[(row + 2) * WIDTH + col - 2];
                    let m = input[(row + 1) * WIDTH + col - 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                if let Some(&S) = input.get((row + 3) * WIDTH + col) {
                    let a = input[(row + 2) * WIDTH + col];
                    let m = input[(row + 1) * WIDTH + col];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                if let Some(&S) = input.get((row + 3) * WIDTH + col + 3) {
                    let a = input[(row + 2) * WIDTH + col + 2];
                    let m = input[(row + 1) * WIDTH + col + 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };
            }
        }
    }

    res
}

#[aoc(day4, part2, AoCS)]
pub fn part2(input: &str) -> u64 {
    const X_MAS_SUM: u16 = 2 * M as u16 + 2 * S as u16;
    let input = input.as_bytes();
    let mut res = 0;

    // Hint to compiler that input size is known
    input.get(WIDTH * HEIGHT - 2).unwrap();

    let mut top = &input[..WIDTH - 1];
    let mut cur = &input[WIDTH..2 * WIDTH - 1];
    let mut bot;

    for row in (2 * WIDTH..WIDTH * HEIGHT).step_by(WIDTH) {
        bot = &input[row..row + WIDTH - 1];
        for col in 1..WIDTH - 2 {
            let tl = top[col - 1] as u16;
            let tr = top[col + 1] as u16;
            let bl = bot[col - 1] as u16;
            let br = bot[col + 1] as u16;
            // Since input letters are very limited, just summing them all up is enough to check all MAS combinations
            let v = (tl + tr + bl + br == X_MAS_SUM && tl != br) as u64;

            // Don't ask lol, moving if here gives 350% boost
            if cur[col] == A {
                res += v;
            }
        }
        top = cur;
        cur = bot;
    }

    res
}

#[aoc(day4, part2, original)]
pub fn part2_original(input: &str) -> u64 {
    const X_MAS_SUM: u16 = 2 * M as u16 + 2 * S as u16;
    let input = input.as_bytes();
    let mut res = 0;

    // Hint to compiler that input size is known
    input.get(WIDTH * HEIGHT - 2).unwrap();

    for i in WIDTH + 1..WIDTH * (HEIGHT - 1) - 2 {
        let tl = input[i - WIDTH - 1] as u16;
        let tr = input[i - WIDTH + 1] as u16;
        let bl = input[i + WIDTH - 1] as u16;
        let br = input[i + WIDTH + 1] as u16;
        // Since input letters are very limited, just summing them all up is enough to check all MAS combinations
        let v = (tl + tr + bl + br == X_MAS_SUM && tl != br) as u64;

        // Don't ask lol, moving if here gives 350% boost
        if input[i] == A {
            res += v;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY4_INPUT: &'static str = include_str!("../input/2024/day4.txt");

    #[test]
    fn part1_input() {
        assert_eq!(part1(DAY4_INPUT), 2464);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(DAY4_INPUT), 1982);
    }
}
