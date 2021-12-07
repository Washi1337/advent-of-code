use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

pub struct Input {
    positions: Vec<isize>,
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;

    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf)?;

    let numbers: Vec<isize> = buf
        .split(',')
        .map(|x| x.parse::<isize>().expect("Expected number"))
        .collect();

    Ok(Input {
        positions: numbers,
    })
}

fn get_minimum_fuel_short_circuit(input: &Input, fuel_cost: fn(isize) -> isize) -> isize {    
    let max_pos = *input.positions.iter().max().unwrap();

    let mut best: isize = isize::MAX;

    for dest in 0..max_pos {
        let mut total = 0;

        for &pos in input.positions.iter() {
            total += fuel_cost((pos - dest).abs());
            if total >= best {
                total = best;
                break
            }
        }

        best = total;
    }

    best
}

fn get_minimum_fuel_naive(input: &Input, fuel_cost: fn(isize) -> isize) -> isize {
    let max_pos = *input.positions.iter().max().unwrap();

    (0..max_pos)
        .map(|dest|{
            let total_fuel: isize = input
                .positions
                .iter()
                .map(|&pos| fuel_cost((pos - dest).abs()))
                .sum();
            total_fuel
        })
        .min()
        .unwrap()
}

pub fn part1(input: &Input) -> isize {
    get_minimum_fuel_short_circuit(&input, |distance| distance)
}

pub fn part2(input: &Input) -> isize {
    get_minimum_fuel_short_circuit(&input, |distance| distance * (distance + 1) / 2)
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

// Parse: (time: 107us)
// Solution 1: 348996 (time: 487us)
// Solution 2: 98231647 (time: 1786us)