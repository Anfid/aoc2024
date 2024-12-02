use anyhow::{anyhow, Result};
use clap::Parser;
use std::io::{BufRead, Read};

fn dispatch(day: u8, part: u8, input: &str) -> Result<String> {
    match (day, part) {
        (1, 1) => aoc2024::day1::part1_safe(input).map(|res| res.to_string()),
        (1, 2) => aoc2024::day1::part2_safe(input).map(|res| res.to_string()),
        (2, 1) => Ok(aoc2024::day2::part1(input)).map(|res| res.to_string()),
        (2, 2) => Ok(aoc2024::day2::part2(input)).map(|res| res.to_string()),
        _ => Err(anyhow!("no solution for day {day} part {part}")),
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let mut stdin = std::io::stdin().lock();
    let mut buf = String::new();

    let is_interactive = atty::is(atty::Stream::Stdin);
    let (day, part, input) = if is_interactive {
        let day = if let Some(day) = cli.day {
            day
        } else {
            eprint!("Day: ");
            buf.clear();
            stdin.read_line(&mut buf)?;
            buf.trim().parse()?
        };
        let part = if let Some(part) = cli.part {
            part
        } else {
            eprint!("Part: ");
            buf.clear();
            stdin.read_line(&mut buf)?;
            buf.trim().parse()?
        };
        let input_path = if let Some(ref path) = cli.input {
            Some(path.as_str())
        } else if cli.stdin {
            None
        } else {
            eprint!("Input (leave empty for STDIN): ");
            buf.clear();
            stdin.read_line(&mut buf)?;
            let path = buf.trim();
            if path.is_empty() {
                None
            } else {
                Some(path)
            }
        };
        let input = if let Some(input_path) = input_path {
            std::fs::read_to_string(input_path)?
        } else {
            eprintln!("Puzzle input (press Ctrl+D when done):");
            buf.clear();
            while stdin.read_line(&mut buf)? != 0 {}
            eprintln!();
            buf
        };

        (day, part, input)
    } else {
        let (Some(day), Some(part)) = (cli.day, cli.part) else {
            if cli.stdin || cli.input.is_some() {
                anyhow::bail!("DAY or PART arguments are missing")
            } else {
                anyhow::bail!("DAY, PART or INPUT arguments are missing")
            }
        };
        let input = if let Some(input_path) = cli.input {
            std::fs::read_to_string(input_path)?
        } else {
            buf.clear();
            stdin.read_to_string(&mut buf)?;
            buf
        };

        (day, part, input)
    };

    let iterations = cli.iterations.unwrap_or(1);

    let start = cli.time.then(|| std::time::Instant::now());
    let mut result = String::new();
    for _ in 0..iterations {
        result = dispatch(day, part, &input)?;
    }
    let duration = start.map(|s| s.elapsed() / iterations);

    println!("{result}");

    if let Some(duration) = duration {
        println!("Time: {duration:?}");
    }

    Ok(())
}

fn main() {
    let result = run();

    if let Err(err) = result {
        eprintln!("\x1b[1;31merror:\x1b[0m {err}");
        std::process::exit(1)
    }
}

#[derive(Debug, Parser)]
struct Cli {
    /// Run solution for day [1..=25]
    #[clap(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    /// Run solution for part [1..=2] of selected day
    #[clap(value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Read input file at <PATH>
    #[clap(long, short, name = "PATH", group = "input")]
    input: Option<String>,

    /// Read input from STDIN
    #[clap(long, group = "input")]
    stdin: bool,

    /// Measure solution execution time
    #[clap(long, short)]
    time: bool,

    /// Measure solution execution time
    #[clap(long, short, value_parser = clap::value_parser!(u32).range(1..))]
    iterations: Option<u32>,
}
