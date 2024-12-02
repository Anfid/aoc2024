use anyhow::Result;
use aoc_runner_derive::aoc;

#[aoc(day2, part1, AoCS)]
pub fn part1(input: &str) -> usize {
    part1_naive(input).unwrap()
}

#[derive(Debug, PartialEq, Eq)]
enum ReportStatus {
    Uninitialized,
    Unknown(i64),
    Descending(i64),
    Ascending(i64),
}

#[aoc(day2, part1, naive)]
pub fn part1_naive(input: &str) -> Result<usize> {
    let reports = parse(input)?;
    let result = reports
        .into_iter()
        .filter(|report| {
            report
                .into_iter()
                .try_fold(ReportStatus::Uninitialized, |acc, &lvl| match acc {
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
                })
                .is_ok()
        })
        .count();
    Ok(result)
}

#[aoc(day2, part2, AoCS)]
pub fn part2(input: &str) -> usize {
    part2_naive(input).unwrap()
}

#[aoc(day2, part2, naive)]
pub fn part2_naive(input: &str) -> Result<usize> {
    let reports = parse(input)?;
    let result = reports
        .into_iter()
        .filter(|report| {
            for i in 0..report.len() {
                let mut r = report.clone();
                r.remove(i);
                if r.into_iter()
                    .try_fold(ReportStatus::Uninitialized, |acc, lvl| match acc {
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
                    })
                    .is_ok()
                {
                    return true;
                }
            }
            false
        })
        .count();
    Ok(result)
}

pub fn parse(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|report| {
            report
                .split_ascii_whitespace()
                .map(|level| level.parse().map_err(Into::into))
                .collect()
        })
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
        assert_eq!(part2_naive(DAY2_INPUT).unwrap(), 4);
    }
}
