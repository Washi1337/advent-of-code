use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

const WORD_LENGTH: usize = 8;
const LETTER_SIZE: Vector2 = Vector2(5, 6);
const WORD_STRIDE: usize = LETTER_SIZE.0 * WORD_LENGTH;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2(usize, usize);

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

pub struct Fold {
    axis: Axis,
    position: usize,
}

pub struct Input {
    points: Vec<Vector2>,
    folds: Vec<Fold>,
}

impl Vector2 {
    pub fn from_str(s: &str) -> Self {
        let mut split = s.split(',');
        let x = split
            .next()
            .expect("Expected X component.")
            .parse::<usize>()
            .expect("X component needs to be an integer.");
        let y = split
            .next()
            .expect("Expected Y component.")
            .parse::<usize>()
            .expect("Y component needs to be an integer.");
        Self(x, y)
    }
}

impl Fold {
    pub fn from_str(s: &str) -> Self {
        let equals_index = s.find('=').expect("Expected '='");
        let axis = match s.chars().nth("fold along ".len()).unwrap() {
            'x' => Axis::X,
            'y' => Axis::Y,
            _ => panic!("Unexpected axis."),
        };

        let position = s[equals_index + 1..]
            .parse::<usize>()
            .expect("Expected numerical position");
        Self { axis, position }
    }
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let lines = BufReader::new(file).lines();

    let mut points = Vec::new();
    let mut folds = Vec::new();

    let mut is_parsing_points = true;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            is_parsing_points = false;
        } else if is_parsing_points {
            points.push(Vector2::from_str(line.as_str()));
        } else {
            folds.push(Fold::from_str(line.as_str()));
        }
    }

    Ok(Input { points, folds })
}

pub fn part1(input: &Input) -> usize {
    // Lazy implementation...

    let fold = &input.folds[0];

    let mut remaining = HashSet::new();
    for &point in input.points.iter() {
        let new_point = if fold.axis == Axis::X {
            if point.0 > fold.position {
                Vector2(fold.position - (point.0 - fold.position), point.1)
            } else {
                point
            }
        } else {
            if point.1 > fold.position {
                Vector2(point.0, fold.position - (point.1 - fold.position))
            } else {
                point
            }
        };

        remaining.insert(new_point);
    }

    remaining.len()
}

pub fn part2(input: &Input) -> String {
    // Step 1: Folding:
    //  Key observation 1:
    //  A fold on the X axis only affects the X coordinate of all points, and same for Y.
    //  The X translations do NOT affect the Y translations in any shape or form, and vice versa.
    //   => We can treat the X and Y coordinates as separate numbers, and treat their foldings
    //      in **complete isolation** without any problems.
    //
    //  Idea is to create a mapping f such that f(i) = final position of the 1D coordinate i.
    //  This way, we cansimply map all points in the input and immediately get to the final output
    //  in one step.
    //
    //  Start with the identity map f, i.e. f(i) = i. Then we can iterate backwards all folds and update
    //  all the values in the mapping.
    //
    //  Key observation 2:
    //  Folding should always end up in 8 characters of each 5x6 dots.
    //   => We only need to initialize these points in our initial identity mapping f.
    //      Since we treat the components in isolation, this is really cheap (only WORD_STRIDE (=40) X mappings, and LETTER_SIZE.1 (=6) Y mappings).
    //
    //  Example:
    //  Let's assume a fold on index 4 followed by a fold on index 2 on coordinates 0 to 8.
    //
    //  Initial identity map:
    //     i | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    //  f(i) | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    //
    //  After applying fold 2:
    //     i | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    //  f(i) | 0 | 1 | 1 | 0 | 4 | 5 | 6 | 7 |
    //
    //  After applying fold 4:
    //     i | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    //  f(i) | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 0 |
    //
    // Step 2: OCR:
    //  Since letters are 5x6 large, they only need 30-bits.
    //  => Letters can be represented using a 32-bit integer. e.g.:
    //
    //   .##..   -> 01100
    //   #..#.   -> 10010
    //   #..#.   -> 10010
    //   ####.   -> 11110
    //   #..#.   -> 10010
    //   #..#.   -> 10010
    //
    //  => A is 0b01100_10010_10010_11110_10010_10010
    //
    //  However, if we reverse the bits, we can get even quicker lookups
    //  due to fewer calculations being reuqired to "draw" the bits in
    //  the u32.
    //
    //  => Final hash for A is 0b01001_01001_01111_01001_01001_00110

    // Set up translation tables.
    let mut x_translations = [0u8; 1500];
    let mut y_translations = [0u8; 1500];

    // Initialize identity mappings.
    for i in 0..WORD_STRIDE {
        x_translations[i] = i as u8;
    }
    for i in 0..LETTER_SIZE.1 {
        y_translations[i] = i as u8;
    }

    // Apply all folds in reverse order.
    for fold in input.folds.iter().rev() {
        if fold.axis == Axis::X {
            for i in 0..=fold.position {
                x_translations[fold.position + i] = x_translations[fold.position - i];
            }
        } else {
            for i in 0..=fold.position {
                y_translations[fold.position + i] = y_translations[fold.position - i];
            }
        }
    }

    // Map all points to their new locations.
    let translated_points = input
        .points
        .iter()
        .map(|p| Vector2(x_translations[p.0] as usize, y_translations[p.1] as usize));

    // "Draw" letters (aka construct letter hashes).
    let mut letter_hashes = [0u32; WORD_LENGTH];
    translated_points.for_each(|p| {
        let letter_index = p.0 / LETTER_SIZE.0;
        let letter_column = p.0 % LETTER_SIZE.0;

        let bit_index = p.1 * LETTER_SIZE.0 + letter_column;
        letter_hashes[letter_index] |= 1 << bit_index;
    });

    // OCR
    let mut result = String::with_capacity(WORD_LENGTH);
    for i in 0..letter_hashes.len() {
        result.push(hash_to_letter(letter_hashes[i]).unwrap_or('?'));
    }

    result
}

fn hash_to_letter(hash: u32) -> Option<char> {
    match hash {
        0b01001_01001_01111_01001_01001_00110 => Some('A'),
        0b00111_01001_01001_00111_01001_00111 => Some('B'),
        0b00110_01001_00001_00001_01001_00110 => Some('C'),
        0b01111_00001_00001_00111_00001_01111 => Some('E'),
        0b00001_00001_00001_00111_00001_01111 => Some('F'),
        0b01110_01001_01101_00001_01001_00110 => Some('G'),
        0b00110_01001_01000_01000_01000_01100 => Some('J'),
        0b01001_00101_00101_00011_00101_01001 => Some('K'),
        0b00001_00001_00111_01001_01001_00111 => Some('P'),
        0b00110_01001_01001_01001_01001_01001 => Some('U'),
        0b01111_00001_00010_00100_01000_01111 => Some('Z'),
        _ => None,
    }
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

// Parse: (time: 223us)
// Solution 1: 788 (time: 247us)
// Solution 2: KJBKEUBG (time: 4us)
//
// Benchmarked:
// part 1 (real)           time:   [46.454 us 46.526 us 46.602 us]
// part 2 (real)           time:   [3.6818 us 3.6940 us 3.7082 us]