use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant, fmt::Display,
};

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

#[derive(Clone, Copy)]
pub struct Vector2(isize, isize);

impl Vector2 {
    /// Translates the index into a position on an energy map.
    pub fn from_index(index: usize) -> Self {
        Self((index % MAP_WIDTH) as isize, (index / MAP_WIDTH) as isize)
    }

    /// Translates the position into an index within the raw grid of an energy map.
    pub fn to_index(&self) -> usize {
        self.1 as usize * MAP_WIDTH + self.0 as usize
    }
}

#[derive(Clone)]
pub struct EnergyMap {
    grid: [u8; MAP_WIDTH * MAP_HEIGHT]
}

impl EnergyMap {
    fn new() -> Self {
        Self {
            grid: [0u8; MAP_WIDTH * MAP_HEIGHT]
        }
    }
    
    pub fn get(&self, location: Vector2) -> u8 {
        self.grid[location.to_index()]
    }

    pub fn set(&mut self, location: Vector2, value: u8) {
        self.grid[location.to_index()] = value;
    }

    pub fn step(&mut self) -> usize {
        let mut agenda = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
        self.step_reuse_stack(&mut agenda)
    }
    
    pub fn step_reuse_stack(&mut self, agenda: &mut Vec<Vector2>) -> usize {
        // Step 1: Increase all energy levels.
        for i in 0..self.grid.len() {
            self.grid[i] += 1;

            // If we are flashing after the increase, store the position for processing.
            if self.grid[i] > 9 {
                agenda.push(Vector2::from_index(i));
            }
        }

        let mut count = 0;

        // Step 2: Flash and ripple through DFS.
        while !agenda.is_empty() {

            // Get current position to process.
            let pos = agenda.pop().unwrap();

            // If we are not flashing, just ignore.
            if self.get(pos) <= 9 {
                continue;
            }

            // We are flashing at this position, reset to 0.
            self.set(pos, 0);

            // Register that we flashed.
            count += 1;

            // Schedule neighbours for processing.
            for dy in -1..=1 {
                // Check if we go out of bounds in Y direction.
                let pos_y = pos.1 + dy;
                if pos_y < 0 || pos_y >= MAP_HEIGHT as isize {
                    continue;
                }

                for dx in -1..=1 {
                    // Check if we go out of bounds in X direction.
                    let pos_x = pos.0 + dx;
                    if pos_x < 0 || pos_x >= MAP_HEIGHT as isize || (dy == 0 && dx == 0) {
                        continue;
                    }
                    
                    // Schedule if the neighbour level isn't reset before.
                    let new_pos = Vector2(pos_x, pos_y);
                    let level = self.get(new_pos);

                    if level > 0 {
                        self.set(new_pos, level + 1);
                        agenda.push(new_pos);
                    }
                }
            }
        }

        count
    }
}

impl Display for EnergyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..MAP_WIDTH {
            for x in 0..MAP_HEIGHT {
                write!(f, "{:>3}", self.get(Vector2(x as isize, y as isize)))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct Input {
    map: EnergyMap
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let mut map = EnergyMap::new();
    BufReader::new(file)
        .lines()
        .enumerate()
        .for_each(|(y, line)| 
            line
                .expect("Expected a line")
                .as_bytes()
                .iter()
                .enumerate()
                .for_each(|(x, &b)| map.set(Vector2(x as isize, y as isize), b - 0x30))
        );

    Ok(Input { map })
}

pub fn part1(input: &Input) -> usize {
    let mut agenda = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
    let mut map = input.map.clone();
    
    (0..100).map(|_| map.step_reuse_stack(&mut agenda)).sum()
}

pub fn part2(input: &Input) -> usize {
    let mut agenda = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
    let mut map = input.map.clone();

    (0..).find_map(|i| {
        if map.step_reuse_stack(&mut agenda) == MAP_WIDTH * MAP_HEIGHT {
            Some(i + 1)
        } else {
            None
        }
    }).unwrap()
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

// Parse: (time: 125us)
// Solution 1: 1673 (time: 73us)
// Solution 2: 279 (time: 183us)