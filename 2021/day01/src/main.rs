use std::{fs::File, io::{BufReader, BufRead}, time::Instant};


fn part1(lines: &Vec<usize>) -> usize {
    lines.iter()
        .skip(1)
        .enumerate()
        .map(|(i, line)| if lines[i] < *line { 1 } else { 0 })
        .sum()
}

fn part2(lines: &Vec<usize>) -> usize {
    let sums: Vec<usize> = lines.iter()
        .skip(2)
        .enumerate()
        .map(|(i, line)| lines[i] + lines[i + 1] + *line)
        .collect();

    sums.iter()
        .skip(1)
        .enumerate()
        .map(|(i, line)| if sums[i] < *line { 1 } else { 0 })
        .sum()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines: Vec<usize> = BufReader::new(file).lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();
    
    let now = Instant::now();
    let result1 = part1(&lines);
    let elapsed1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&lines);
    let elapsed2 = now.elapsed();

    println!("{} (time: {})", result1, elapsed1.as_nanos());
    println!("{} (time: {})", result2, elapsed2.as_nanos());
    Ok(())
}
