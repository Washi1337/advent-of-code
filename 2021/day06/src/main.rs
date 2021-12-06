use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

// Key observations:
//
// - We can group (= keep track of count of) all fish with the same timer value,
//   and calculate their total contribution to the population by simple multiplication
//   => We just need to store the counts of all fish with the same timer value.
//
// - A fish only has effect on the state after 7 days (or 9 days).
//   => A fish at timer t affects the nr of fish at timer (t+7)%9
//
// - If fish triggers update, then after 7 days its back at the same timer value.
//   => If we iterate all days,  "resetting" timers of the previous fish doesn't affect
//      final outcome, because after 7 days we end up at the same value anyways.
//
// => We can implement the entire thing as a simple feedback shift register that just increases
//    the number of fish at (t+7)%9 by the number of fish with timer (t % 9).
//    Total fish count is then just sum of all counts.

pub struct Input {
    initial_state: Vec<usize>,
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;

    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf)?;

    let numbers: Vec<usize> = buf
        .split(',')
        .map(|x| x.parse::<usize>().expect("Expected number"))
        .collect();

    Ok(Input {
        initial_state: numbers,
    })
}

pub fn simulate(input: &Input, days: usize) -> usize {
    let mut fish_counts = [0usize; 9];

    for &timer in input.initial_state.iter() {
        fish_counts[timer] += 1;
    }

    for day in 0..days {
        fish_counts[(day + 7) % 9] += fish_counts[day % 9];
    }

    fish_counts.iter().sum()
}

pub fn part1(input: &Input) -> usize {
    simulate(&input, 80)
}

pub fn part2(input: &Input) -> usize {
    simulate(&input, 256)
    // 0
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

// Parse: (time: 139us)
// Solution 1: 394994 (time: 0us)
// Solution 2: 1765974267455 (time: 0us)
