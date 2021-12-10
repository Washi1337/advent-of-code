use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

pub struct Input {
    lines: Vec<String>,
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    Ok(Input { lines })
}

pub fn part1(input: &Input) -> usize {
    let mut stack = Vec::with_capacity(input.lines[0].len());

    input
        .lines
        .iter()
        .filter_map(|line| {
            stack.clear();

            line.chars().find_map(|c| {
                let e = match c {
                    ')' => Some(('(', 3)),
                    ']' => Some(('[', 57)),
                    '}' => Some(('{', 1197)),
                    '>' => Some(('<', 25137)),
                    _ => None,
                };

                if let Some(expected) = e {
                    if let Some(actual) = stack.pop() {
                        if expected.0 != actual {
                            return Some(expected.1);
                        }
                    }
                } else {
                    stack.push(c);
                }

                None
            })
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut stack = Vec::with_capacity(input.lines[0].len());

    let mut scores: Vec<usize> = input
        .lines
        .iter()
        .filter_map(|line| {
            stack.clear();

            for c in line.chars() {
                let e = match c {
                    ')' => Some('('),
                    ']' => Some('['),
                    '}' => Some('{'),
                    '>' => Some('<'),
                    _ => None,
                };

                if let Some(expected) = e {
                    if let Some(actual) = stack.pop() {
                        if expected != actual {
                            return None;
                        }
                    }
                } else {
                    stack.push(c);
                }
            }

            Some(stack.iter().rev().fold(0, |acc, c| {
                let score = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };

                acc * 5 + score
            }))
        })
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    let input = parse_input("input.txt")?;
    let time_parse = now.elapsed();
    println!("Parse: (time: {}us)", time_parse.as_micros());

    let now = Instant::now();
    let result1 = part1(&input);
    let time1 = now.elapsed();
    println!("Solution 1: {} (time: {}us)", result1, time1.as_micros());

    let now = Instant::now();
    let result2 = part2(&input);
    let time2 = now.elapsed();
    println!("Solution 2: {} (time: {}us)", result2, time2.as_micros());

    Ok(())
}

// Parse: (time: 83us)
// Solution 1: 389589 (time: 48us)
// Solution 2: 1190420163 (time: 62us)
