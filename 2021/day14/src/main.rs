use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

pub struct InsertionRule {
    pair: (u8, u8),
    insertion: u8,
}

pub struct Input {
    template: Vec<u8>,
    insertion_rules: Vec<InsertionRule>,
}

impl InsertionRule {
    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        Self {
            pair: (bytes[0] - 'A' as u8, bytes[1] - 'A' as u8),
            insertion: bytes[6] - 'A' as u8,
        }
    }
}

impl Display for InsertionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} -> {}",
            (self.pair.0 + 'A' as u8) as char,
            (self.pair.1 + 'A' as u8) as char,
            (self.insertion + 'A' as u8) as char
        )
    }
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let mut lines = BufReader::new(file).lines();

    let template = lines
        .next()
        .expect("Expected polymer template")?
        .as_bytes()
        .iter()
        .map(|b| b - 'A' as u8)
        .collect();

    lines.next().expect("Expected blank line")?;

    let insertion_rules: Vec<InsertionRule> = lines
        .map(|line| {
            InsertionRule::from_str(line.expect("Expected a pair insertion rule.").as_str())
        })
        .collect();

    Ok(Input {
        template,
        insertion_rules,
    })
}

pub fn simulate(input: &Input, steps: usize) -> usize {
    const ALPHABET_SIZE: usize = 26;

    fn pair_to_index(p: &(u8, u8)) -> usize {
        p.0 as usize * ALPHABET_SIZE + p.1 as usize
    }

    // Stores the current state of the polymer as counts of every pair.
    // A pair is referenced by the index p.0 * ALPHABET_SIZE + p.1.
    let mut pair_counts = [0usize; ALPHABET_SIZE * ALPHABET_SIZE];

    // Initialize pair counts with the polymer template.
    input.template.windows(2).for_each(|p| {
        pair_counts[pair_to_index(&(p[0], p[1]))] += 1;
    });

    // Stores a mapping from pair to a pair of new pairs that gets produced after 
    // the insertion has taken place.
    let mut pair_productions = [0u32; ALPHABET_SIZE * ALPHABET_SIZE];

    for rule in input.insertion_rules.iter() {
        // An insertion rule AB -> C produces from one pair AB two new pairs AC and CB.
        let old_pair = pair_to_index(&rule.pair);
        let new_pair_1 = pair_to_index(&(rule.pair.0, rule.insertion));
        let new_pair_2 = pair_to_index(&(rule.insertion, rule.pair.1));

        // Register the production of the two new pairs.
        pair_productions[old_pair] = (new_pair_1 | new_pair_2 << 16) as u32;
    }
    
    // Iterate all steps.
    for _ in 0..steps {
        // Create a new polymer.
        let mut new_counts = [0usize; ALPHABET_SIZE * ALPHABET_SIZE];

        for rule in input.insertion_rules.iter() {
            // Get the number of current instances of the pair in the polymer.
            let p_index = pair_to_index(&rule.pair);
            let count = pair_counts[p_index];

            // Get new pairs.
            let new_pairs = pair_productions[p_index];
            let new_pair1 = (new_pairs & 0xFFFF) as usize;
            let new_pair2 = ((new_pairs >> 16) & 0xFFFF) as usize;

            // Add them to the polymer.
            new_counts[new_pair1] += count;
            new_counts[new_pair2] += count;
        }

        // Swap old polymer with new polymer.
        pair_counts.copy_from_slice(&new_counts);
    }

    // Count all elements in the polymer, and sort them by character.
    // We only need to count one character in the pair, since all characters
    // are part of two pairs.
    let mut element_counts = [0usize; ALPHABET_SIZE];
    for p_index in 0..pair_counts.len() {
        element_counts[p_index % ALPHABET_SIZE] += pair_counts[p_index];
    }

    // Off-by-one, first character in the polymer is an exception to the counting rule.
    element_counts[input.template[0] as usize] += 1;    

    // Find min-max counts.
    let mut min = usize::MAX;
    let mut max = 0usize;
    for count in element_counts {
        if count > max {
            max = count;
        } else if count < min && count > 0 {
            min = count;
        }
    }

    // Final solution.
    max - min
}

pub fn part1(input: &Input) -> usize {
    simulate(&input, 10)
}

pub fn part2(input: &Input) -> usize {
    simulate(&input, 40)
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

// Parse: (time: 86us)
// Solution 1: 2768 (time: 13us)
// Solution 2: 2914365137499 (time: 12us)
//
// Benchmarked:
// part 1 (real)           time:   [3.2131 us 3.2225 us 3.2332 us]
// part 2 (real)           time:   [10.908 us 10.937 us 10.969 us]
