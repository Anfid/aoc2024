use crate::parsers::BytesAsciiExt;
use anyhow::Result;
use aoc_runner_derive::aoc;

#[aoc(day2, part1, AoCS)]
pub fn part1(input: &str) -> usize {
    let bytes = input.as_bytes();
    let result = bytes.ascii_lines().filter(verify_report).count();
    result
}

fn parse_input_digit(num: &[u8]) -> i32 {
    (match num {
        &[tens, ones] => (tens - b'0') * 10 + ones - b'0',
        &[ones] => ones - b'0',
        _ => unsafe { std::hint::unreachable_unchecked() },
    }) as i32
}

#[derive(Debug, PartialEq, Eq)]
enum ReportStatus {
    Uninitialized,
    Unknown(i32),
    Descending(i32),
    Ascending(i32),
}

fn verify_report(&report: &&[u8]) -> bool {
    report
        .ascii_words()
        .map(parse_input_digit)
        .try_fold(ReportStatus::Uninitialized, analyze_next_lvl)
        .is_ok()
}

fn analyze_next_lvl(acc: ReportStatus, lvl: i32) -> Result<ReportStatus, ()> {
    match acc {
        ReportStatus::Uninitialized => Ok(ReportStatus::Unknown(lvl)),
        ReportStatus::Unknown(prev) => match prev - lvl {
            -3..=-1 => Ok(ReportStatus::Ascending(lvl)),
            1..=3 => Ok(ReportStatus::Descending(lvl)),
            _ => Err(()),
        },
        ReportStatus::Descending(prev) => {
            if let 1..=3 = prev - lvl {
                Ok(ReportStatus::Descending(lvl))
            } else {
                Err(())
            }
        }
        ReportStatus::Ascending(prev) => {
            if let 1..=3 = lvl - prev {
                Ok(ReportStatus::Ascending(lvl))
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ReportStatusDampened {
    Descending(bool, i32, i32),
    Ascending(bool, i32, i32),
}

fn analyze_next_lvl_dampened(
    acc: ReportStatusDampened,
    curr: i32,
) -> Result<ReportStatusDampened, ()> {
    match acc {
        ReportStatusDampened::Descending(corrected, postpred, pred) => {
            if let 1..=3 = pred - curr {
                Ok(ReportStatusDampened::Descending(corrected, pred, curr))
            } else if !corrected {
                if matches!(postpred - curr, 1..=3) {
                    Ok(ReportStatusDampened::Descending(true, pred, curr))
                } else {
                    Ok(ReportStatusDampened::Descending(true, postpred, pred))
                }
            } else {
                Err(())
            }
        }
        ReportStatusDampened::Ascending(corrected, postpred, pred) => {
            if let 1..=3 = curr - pred {
                Ok(ReportStatusDampened::Ascending(corrected, pred, curr))
            } else if !corrected {
                if matches!(curr - postpred, 1..=3) {
                    Ok(ReportStatusDampened::Ascending(true, pred, curr))
                } else {
                    Ok(ReportStatusDampened::Ascending(true, postpred, pred))
                }
            } else {
                Err(())
            }
        }
    }
}

fn report_trend_is_ascending(x: [i32; 4]) -> bool {
    (x[0] < x[1]) as u32 + (x[1] < x[2]) as u32 + (x[2] < x[3]) as u32 > 1
}

fn verify_report_dampened(&report: &&[u8]) -> bool {
    let mut report = report.ascii_words().map(parse_input_digit);
    let head = unsafe {
        [
            report.next().unwrap_unchecked(),
            report.next().unwrap_unchecked(),
            report.next().unwrap_unchecked(),
            report.next().unwrap_unchecked(),
        ]
    };
    let status = if report_trend_is_ascending(head) {
        if !matches!(head[1] - head[0], 1..=3) && matches!(head[2] - head[1], 1..=3) {
            ReportStatusDampened::Ascending(true, head[1] - 1, head[1] - 1)
        } else {
            ReportStatusDampened::Ascending(false, head[0], head[0])
        }
    } else {
        if !matches!(head[0] - head[1], 1..=3) && matches!(head[1] - head[2], 1..=3) {
            ReportStatusDampened::Descending(true, head[1] + 1, head[1] + 1)
        } else {
            ReportStatusDampened::Descending(false, head[0], head[0])
        }
    };
    head[1..]
        .into_iter()
        .copied()
        .chain(report)
        .try_fold(status, analyze_next_lvl_dampened)
        .is_ok()
}

#[aoc(day2, part2, AoCS)]
pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    let result = bytes.ascii_lines().filter(verify_report_dampened).count();
    result
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let bytes = input.as_bytes();
    bytes
        .ascii_lines()
        .map(|report| report.ascii_words().map(parse_input_digit).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY2_INPUT: &'static str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DAY2_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DAY2_INPUT), 4);
    }
}
