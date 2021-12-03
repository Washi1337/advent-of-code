use std::{fs::File, io::{BufReader, BufRead}, time::Instant};


const BIT_LENGTH: usize = 12;
const BIT_MASK: usize = (1 << BIT_LENGTH) - 1;


fn part1(input: &Vec<u16>) -> usize {
    let mut gamma: usize = 0;

    for i in 0..BIT_LENGTH {
        let mut zeroes = 0;
        let mut ones = 0;
        
        for x in input {
            if x & (1 << i) == 0 {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }

        if ones > zeroes {
            gamma |= 1 << i;
        }
    }

    let epsilon = !gamma & BIT_MASK;

    gamma * epsilon
}


fn part2(input: &Vec<u16>) -> usize {
    let oxygen = do_filter(&input, |a, b| a > b);
    let co2 = do_filter(&input, |a, b| a <= b);

    oxygen * co2
}


fn do_filter(input: &Vec<u16>, criteria: fn(usize, usize) -> bool) -> usize {
    let mut working_set = input.clone();
    let mut set0 = Vec::with_capacity(working_set.len());
    let mut set1 = Vec::with_capacity(working_set.len());

    for i in (0..BIT_LENGTH).rev() {
        if working_set.len() == 1 {
            break;
        }

        for &x in &working_set {
            if x & (1 << i) == 0 {
                set0.push(x);
            } else {
                set1.push(x);
            }
        }

        working_set.clear();
        working_set.extend(if criteria(set0.len(), set1.len()) {
            &set0
        } else {
            &set1
        });

        set0.clear();
        set1.clear();
    }

    working_set[0] as usize
}


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let input: Vec<u16> = BufReader::new(file)
        .lines()
        .map(|x| u16::from_str_radix(x.unwrap().as_str(), 2).unwrap())
        .collect();
    
    let now = Instant::now();
    let result1 = part1(&input);
    let elapsed1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&input);
    let elapsed2 = now.elapsed();

    println!("Part1: {} (time: {})", result1, elapsed1.as_nanos());
    println!("Part2: {} (time: {})", result2, elapsed2.as_nanos());
    Ok(())
}

// Part1: 2035764 (time: 2600)
// Part2: 2817661 (time: 17500)