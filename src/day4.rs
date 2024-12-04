use anyhow::Result;
use aoc_runner_derive::aoc;

const WIDTH: usize = 141;
const HEIGHT: usize = 140;

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

#[aoc(day4, part1, AoCS)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut res = 0;

    for row in 0..HEIGHT {
        for col in 0..WIDTH - 1 {
            if input[row * WIDTH + col] == X {
                if let Some(&S) = input.get(row * WIDTH + col - 3) {
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

                if let Some(&S) = input.get((row - 3) * WIDTH + col - 3) {
                    let a = input[(row - 2) * WIDTH + col - 2];
                    let m = input[(row - 1) * WIDTH + col - 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                if let Some(&S) = input.get((row - 3) * WIDTH + col) {
                    let a = input[(row - 2) * WIDTH + col];
                    let m = input[(row - 1) * WIDTH + col];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                if let Some(&S) = input.get((row - 3) * WIDTH + col + 3) {
                    let a = input[(row - 2) * WIDTH + col + 2];
                    let m = input[(row - 1) * WIDTH + col + 1];
                    if (m, a) == (M, A) {
                        res += 1;
                    }
                };

                // --------------

                if let Some(&S) = input.get((row + 3) * WIDTH + col - 3) {
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

#[aoc(day4, part1, bitmap)]
pub fn part1_bitmap_wip(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut res = 0;
    // (3x<letter>) x8
    let mut bitmap = [[0u32; (WIDTH - 1)]; HEIGHT];

    let mut line;
    for row in 0..HEIGHT {
        line = &input[row * WIDTH..(row + 1) * WIDTH - 1];
        for col in 0..WIDTH - 1 {
            match line[col] {
                X => {
                    bitmap
                        .get_mut(row)
                        .and_then(|row| row.get_mut(col + 1))
                        .map(|map| *map |= 1 << 8 * 3 - 1);
                    bitmap.get_mut(row + 1).map(|row| {
                        row.get_mut(col + 1).map(|map| *map |= 1 << 7 * 3 - 1);
                        row.get_mut(col).map(|map| *map |= 1 << 6 * 3 - 1);
                        row.get_mut(col - 1).map(|map| *map |= 1 << 5 * 3 - 1);
                    });
                    res += (bitmap[row][col] & 0b001001001001).count_ones();
                }
                M => {}
                A => {}
                S => {
                    bitmap
                        .get_mut(row)
                        .and_then(|row| row.get_mut(col + 1))
                        .map(|map| *map |= 1 << 4 * 3 - 1);
                    bitmap.get_mut(row + 1).map(|row| {
                        row.get_mut(col + 1).map(|map| *map |= 1 << 3 * 3 - 1);
                        row.get_mut(col).map(|map| *map |= 1 << 2 * 3 - 1);
                        row.get_mut(col - 1).map(|map| *map |= 1 << 1 * 3 - 1);
                    });
                    res += (bitmap[row][col] & 0b001001001001000000000000).count_ones();
                }
                _ => {}
            }
        }
    }

    res
}

#[aoc(day4, part1, weird)]
pub fn part1_weird(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut res = 0;

    // Hint to compiler that input size is known
    input.get(WIDTH * HEIGHT - 2).unwrap();

    let mut l1 = &input[..WIDTH - 1];
    let mut l2 = &input[WIDTH..2 * WIDTH - 1];
    let mut l3 = &input[2 * WIDTH..3 * WIDTH - 1];
    let mut l4;

    for row_start in (3 * WIDTH..WIDTH * (HEIGHT - 4)).step_by(WIDTH) {
        l4 = &input[row_start..row_start + WIDTH - 1];
        for col in 0..WIDTH - 2 {
            let paths = [
                [
                    input[row_start + col],
                    input[row_start + col + 1],
                    input[row_start + col + 2],
                    input[row_start + col + 3],
                ],
                [
                    input[row_start + col],
                    input[WIDTH + row_start + col + 1],
                    input[2 * WIDTH + row_start + col + 2],
                    input[3 * WIDTH + row_start + col + 3],
                ],
                [
                    input[row_start + col],
                    input[WIDTH + row_start + col],
                    input[2 * WIDTH + row_start + col],
                    input[3 * WIDTH + row_start + col],
                ],
                [
                    input[row_start + col],
                    input[WIDTH + row_start + col - 1],
                    input[2 * WIDTH + row_start + col - 2],
                    input[3 * WIDTH + row_start + col - 3],
                ],
            ];
            let v = match l1[col] {
                X => paths.iter().filter(|p| matches!(p, [X, M, A, S])).count(),
                S => paths.iter().filter(|p| matches!(p, [S, A, M, X])).count(),
                _ => 0,
            };
            //dbg!(
            //    paths
            //        .iter()
            //        .map(|p| std::str::from_utf8(p).unwrap())
            //        .collect::<Vec<_>>(),
            //    v
            //);
            res += v as u64;
        }
        l1 = l2;
        l2 = l3;
        l3 = l4;
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

    fn test_key() {
        todo!()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(DAY4_INPUT), 2464);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DAY4_INPUT), 1982);
    }
}
