use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

/// Represents one signal pattern within the input.
type Signal = u8;

/// Represents a signal pattern paired with its hamming weight.
type WeightedSignal = (Signal, usize);

/// Represents one input line within the input.
pub struct Entry {
    /// The signal pattern configuration.
    patterns: [WeightedSignal; 10],

    /// The observed outputs.
    outputs: [WeightedSignal; 4],
}

/// Represents the input for the puzzle.
pub struct Input {
    entries: Vec<Entry>,
}

/// A structure that keeps track of known signal patterns to their corresponding digits.
pub struct SignalMapping {
    /// A mapping from signals to digits.
    mapping: [usize; 256],

    /// A mapping from digits to known signal pattern masks.
    known_signals: [Signal; 10],
}

impl SignalMapping {
    /// Initializes a new empty signal mapping.
    pub fn new() -> Self {
        Self {
            mapping: [0; 256],
            known_signals: [0u8; 10],
        }
    }

    /// Assigns a signal to a digit.
    pub fn assign(&mut self, signal: Signal, digit: usize) {
        self.mapping[signal as usize] = digit as usize;
        self.known_signals[digit as usize] = signal;
    }

    /// Determines whether the provided digit is already mapped to a signal pattern.
    pub fn has_known_signal(&self, digit: usize) -> bool {
        self.known_signals[digit] != 0
    }

    /// Gets the final number associated with the provided output patterns.
    pub fn get_number(&self, outputs: &[WeightedSignal; 4]) -> usize {
        self.mapping[outputs[0].0 as usize] * 1000
            + self.mapping[outputs[1].0 as usize] * 100
            + self.mapping[outputs[2].0 as usize] * 10
            + self.mapping[outputs[3].0 as usize]
    }
}

impl Entry {
    /// Parses an input entry from a string slice. 
    /// The slice should be in the format: `<patterns> | <outputs>`.
    pub fn from_str(s: &str) -> Self {

        /// Parses a single signal from a string slice.
        fn parse_signal(s: &str) -> WeightedSignal {
            let mut result = 0;

            for c in s.as_bytes() {
                result |= 1 << (c - 97);
            }

            (result, s.len())
        }

        /// Parses a list of signals from a string slice.
        fn parse_signals<const N: usize>(s: &str, buf: &mut [WeightedSignal; N]) {
            let mut split = s.split(' ');
            for i in 0..N {
                buf[i] = parse_signal(split.next().expect("Expected component"));
            }
        }

        let mut patterns = [(0u8, 0usize); 10];
        let mut outputs = [(0u8, 0usize); 4];

        let mut delimeter_split = s.split(" | ");

        parse_signals(
            delimeter_split.next().expect("Expected signal patterns."),
            &mut patterns,
        );
        parse_signals(
            delimeter_split.next().expect("Expected output values."),
            &mut outputs,
        );

        Self { patterns, outputs }
    }

    /// Deduces the digits 1, 4, 7 and 8 from the configuration, and returns a list 
    /// of (partially) parsed numbers from the output.
    pub fn deduce_digits_1478(&self) -> [Option<usize>; 4] {
        let mut result = [None; 4];
        for i in 0..result.len() {
            result[i] = get_number_by_weight(self.outputs[i].1 as usize);
        }
        result
    }

    /// Deduces the full wire configuration, and returns the final number indicated 
    /// by the output digits.
    pub fn deduce_output(&self) -> usize {
        let mut mapping = SignalMapping::new();

        let mut i = 0;
        let mut j = 0;
        let mut weight5 = [0u8; 3];
        let mut weight6 = [0u8; 3];

        // Find the digits 1, 4, 7, 8 first, and presort the unknown signals based on 
        // their hamming weight.
        for signal in self.patterns {
            match get_number_by_weight(signal.1) {
                Some(x) => mapping.assign(signal.0, x),
                None => match signal.1 {
                    5 => {
                        weight5[i] = signal.0;
                        i += 1;
                    }
                    6 => {
                        weight6[j] = signal.0;
                        j += 1;
                    }
                    _ => unreachable!(),
                },
            };
        }

        // For signals with weight 6, it can only be the digits 0, 6 or 9. 
        // - 6 is the only digit that does not have all segments from 1.
        // - 0 is the only one that doesn't have the middle segment, which is present in 4.
        // - 9 remains if both of these conditions are not met.
        for signal in weight6 {
            if (signal & mapping.known_signals[1]) != mapping.known_signals[1] {
                mapping.assign(signal, 6);
            } else if (signal & mapping.known_signals[4]) != mapping.known_signals[4] {
                mapping.assign(signal, 0);
            } else {
                mapping.assign(signal, 9);
            }
        }

        // For signals with weight 5, it can only be the digits 2, 3, 5
        // - 3 has again all segments of 1.
        // - 2 has fewer segments in common with 6 than 5.
        for signal in weight5 {
            if (signal & mapping.known_signals[1]) == mapping.known_signals[1] {
                mapping.assign(signal, 3);
            } else if mapping.has_known_signal(5) || get_weight(signal & mapping.known_signals[6]) == 4 {
                mapping.assign(signal, 2);
            } else {
                mapping.assign(signal, 5);
            }
        }

        // All digit patterns are matched, get final output.
        mapping.get_number(&self.outputs)
    }
}

/// Computes the number of bits set in a 7-bit number.
fn get_weight(x: u8) -> usize {
    (0..7).fold(0, |acc, i| acc + ((x >> i) & 1)) as usize
}

/// Guesses the digit based on the provided hamming weight. This only works for the digits 1, 4, 7 and 8 because
/// they have unique hamming weights.
fn get_number_by_weight(weight: usize) -> Option<usize> {
    match weight {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let lines = BufReader::new(file).lines();

    let entries: Vec<Entry> = lines
        .map(|line| Entry::from_str(line.expect("Expected entry").as_str()))
        .collect();

    Ok(Input { entries })
}

pub fn part1(input: &Input) -> usize {
    input
        .entries
        .iter()
        .map(|e| e.deduce_digits_1478().iter().filter_map(|&x| x).count())
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input.entries.iter().map(|e| e.deduce_output()).sum()
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

// Parse: (time: 262us)
// Solution 1: 416 (time: 0us)
// Solution 2: 1043697 (time: 28us)
