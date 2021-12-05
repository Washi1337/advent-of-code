use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    time::Instant,
};

const BOARD_WIDTH: usize = 5;

const ENDING_MASKS: [u32; BOARD_WIDTH * 2] = [
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001,
];

pub struct Input {
    pub order: Vec<u8>,
    pub boards: Vec<Board>,
}

pub struct Board {
    pub grid: [u8; BOARD_WIDTH * BOARD_WIDTH],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [0u8; BOARD_WIDTH * BOARD_WIDTH],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y * BOARD_WIDTH + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.grid[y * BOARD_WIDTH + x] = value;
    }

    pub fn update_and_get_score(&self, number: u8, marking: &mut u32) -> Option<usize> {
        self.grid
            .iter()
            .position(|&x| x == number) // Search the grid for the number.
            .map_or(None, |index| {
                // Update marking.
                let new_marking = *marking | (1 << index);
                *marking = new_marking;

                // Check if there is any winning row/col and calculate score.
                ENDING_MASKS.iter().find_map(|&ending| {
                    if (new_marking & ending) != ending {
                        // This row/col is not fully marked, no score can be assigned.
                        None
                    } else {
                        // Sum all unmarked fields.
                        let s: usize = (0..self.grid.len())
                            .filter(|&i| ((new_marking >> i) & 1) == 0)
                            .map(|i| self.grid[i] as usize)
                            .sum();

                        // Calculate final score.
                        Some(s * (number as usize))
                    }
                })
            })
    }
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let mut lines = BufReader::new(file).lines();

    let order: Vec<u8> = lines
        .next()
        .expect("Expected random order of numbers.")?
        .split(',')
        .map(|x| x.parse::<u8>().expect("Expected a number in order."))
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let board = parse_board(&mut lines)?;
        boards.push(board);
    }

    Ok(Input {
        order: order,
        boards: boards,
    })
}

fn parse_board(lines: &mut Lines<BufReader<File>>) -> std::io::Result<Board> {
    let mut result = Board::new();

    for y in 0..BOARD_WIDTH {
        let line: Vec<u8> = lines
            .next()
            .expect("Expected line of numbers")?
            .split(' ')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.parse::<u8>().expect("Expected a number in board."))
                }
            })
            .collect();

        for x in 0..BOARD_WIDTH {
            result.set(x, y, line[x]);
        }
    }

    Ok(result)
}

pub fn part1(input: &Input) -> usize {
    let mut markings = vec![0u32; input.boards.len()];

    input
        .order
        .iter()
        .find_map(|&x| {
            input
                .boards
                .iter()
                .enumerate()
                .find_map(|(i, b)| b.update_and_get_score(x, &mut markings[i]))
        })
        .unwrap()
}

pub fn part2(input: &Input) -> usize {
    let mut finished: Vec<bool> = vec![false; input.boards.len()];
    let mut markings = vec![0u32; input.boards.len()];

    let mut last = 0;

    input.order.iter().for_each(|&n| {
        for i in 0..input.boards.len() {
            if finished[i] {
                continue;
            }

            let result = input.boards[i].update_and_get_score(n, &mut markings[i]);
            if let Some(score) = result {
                finished[i] = true;
                last = score;
            }
        }
    });

    last
}

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    let input = parse_input("input.txt")?;
    let time_parse = now.elapsed();

    let now = Instant::now();
    let result1 = part1(&input);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&input);
    let time2 = now.elapsed();

    println!("Parse: (time: {}us)", time_parse.as_micros());
    println!("Solution 1: {} (time: {}us)", result1, time1.as_micros());
    println!("Solution 2: {} (time: {}us)", result2, time2.as_micros());

    Ok(())
}

// Parse: (time: 175us)
// Solution 1: 58838 (time: 14us)
// Solution 2: 6256 (time: 102us)
