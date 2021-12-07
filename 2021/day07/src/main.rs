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

fn get_minimum_fuel_binary(input: &Input, fuel_cost: fn(isize) -> isize) -> isize {

    fn get_total_cost(input: &Input, dest: isize, fuel_cost: fn(isize) -> isize) -> isize {
        input
            .positions
            .iter()
            .map(|&pos| fuel_cost((pos - dest).abs()))
            .sum()
    }

    // Key observation is that if you'd plot the total cost based on position, then you
    // get a graph where the only local minimum == the global minimum (a sink).
    //
    //  => We can do something similar to a binary search. Start in the middle, and check 
    //     by going left and right of the current candidate position which direction will 
    //     decrease the total cost. Stop when both will result in an increase.

    let mut mid_pos = input.positions.iter().sum::<isize>() / input.positions.len() as isize;
    let mut mid_fuel = get_total_cost(&input, mid_pos, fuel_cost);

    loop {
        let left_fuel = get_total_cost(&input, mid_pos - 1, fuel_cost);
        let right_fuel = get_total_cost(&input, mid_pos + 1, fuel_cost);

        if left_fuel < mid_fuel {
            mid_fuel = left_fuel;
            mid_pos -= 1;
        } else if right_fuel < mid_fuel {
            mid_fuel = right_fuel;
            mid_pos += 1;
        } else {
            return mid_fuel;
        }
    }
}

pub fn part1(input: &Input) -> isize {
    get_minimum_fuel_binary(&input, |distance| distance)
}

pub fn part2(input: &Input) -> isize {
    get_minimum_fuel_binary(&input, |distance| distance * (distance + 1) / 2)
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

// Parse: (time: 117us)
// Solution 1: 348996 (time: 69us)
// Solution 2: 98231647 (time: 5us)