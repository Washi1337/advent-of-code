use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant, fmt::Display,
};

const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 100;
const MAX_HEIGHT: u8 = 9;

pub struct HeightMap {
    grid: [u8; MAP_WIDTH * MAP_HEIGHT],
}

#[derive(Debug)]
pub struct Neighbours {
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8
}

/// Represents the input for the puzzle.
pub struct Input {
    map: HeightMap
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2(usize, usize);

impl Vector2 {
    pub fn from_index(index: usize) -> Self {
        Self(index % MAP_WIDTH, index / MAP_WIDTH)
    }

    pub fn to_index(&self) -> usize {
        self.1 * MAP_WIDTH + self.0
    }
}

impl HeightMap {
    pub fn new() -> Self {
        Self {
            grid: [MAX_HEIGHT; MAP_WIDTH * MAP_HEIGHT]
        }
    }

    pub fn get(&self, location: Vector2) -> u8 {
        self.grid[location.to_index()]
    }

    pub fn set(&mut self, location: Vector2, height: u8) {
        self.grid[location.to_index()] = height;
    }

    pub fn get_neighbours(&self, location: Vector2) -> Neighbours {
        let top = if location.1 > 0 {
            self.get(Vector2(location.0, location.1 - 1))
        } else {
            MAX_HEIGHT
        };

        let right = if location.0 < MAP_WIDTH - 1 {
            self.get(Vector2(location.0 + 1, location.1))
        } else {
            MAX_HEIGHT
        };

        let bottom = if location.1 < MAP_HEIGHT - 1 {
            self.get(Vector2(location.0, location.1 + 1))
        } else {
            MAX_HEIGHT
        };

        let left = if location.0 > 0 {
            self.get(Vector2(location.0 - 1, location.1))
        } else {
            MAX_HEIGHT
        };

        Neighbours {
            top,
            right,
            bottom,
            left
        }
    }

    pub fn is_low_point(&self, location: Vector2) -> bool {
        let height = self.get(location);
        let neighbours = self.get_neighbours(location);
        height < neighbours.top 
            && height < neighbours.right 
            && height < neighbours.bottom 
            && height < neighbours.left
    }

    pub fn get_risk_level(&self, location: Vector2) -> usize {
        (self.get(location) + 1) as usize
    }

    pub fn get_basin_size(&self, location: Vector2, agenda: &mut Vec<Vector2>) -> Option<usize> {
        if !self.is_low_point(location) {
            return None;
        }

        let mut size = 0;

        let mut visited = [false; MAP_WIDTH * MAP_HEIGHT];
        agenda.push(location);

        while !agenda.is_empty() {
            let location = agenda.pop().unwrap();
            let index = location.to_index();
            if visited[index] {
                continue;
            }

            visited[index] = true;
            size += 1;

            let height = self.get(location);
            let neighbours = self.get_neighbours(location);
            if neighbours.left != MAX_HEIGHT && neighbours.left > height {
                agenda.push(Vector2(location.0 - 1, location.1))
            }
            if neighbours.right != MAX_HEIGHT && neighbours.right > height {
                agenda.push(Vector2(location.0 + 1, location.1))
            }
            if neighbours.top != MAX_HEIGHT && neighbours.top > height {
                agenda.push(Vector2(location.0, location.1 - 1))
            }
            if neighbours.bottom != MAX_HEIGHT && neighbours.bottom > height {
                agenda.push(Vector2(location.0, location.1 + 1))
            }
        }

        Some(size)
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                write!(f, "{}", self.get(Vector2(x, y)))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let lines = BufReader::new(file).lines();

    let mut map = HeightMap::new();

    lines.enumerate().for_each(|(y, line)| {
        line
            .expect("Expected height map line")
            .as_bytes()
            .iter()
            .map(|&b| b - 0x30)
            .enumerate()
            .for_each(|(x, h)| map.set(Vector2(x, y), h));
    });

    Ok(Input { map })
}

pub fn part1(input: &Input) -> usize {
    (0..MAP_HEIGHT)
        .map(|y|
            (0..MAP_WIDTH)
                .filter_map(|x| {
                    let pos = Vector2(x, y);
                    if input.map.is_low_point(pos) {
                        Some(input.map.get_risk_level(pos))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        )
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut agenda = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);

    let mut sizes: Vec<usize> = (0..MAP_WIDTH * MAP_HEIGHT)
        .filter_map(|i| 
            input.map.get_basin_size(Vector2::from_index(i), &mut agenda)
        )
        .collect();

    sizes.sort_by(|a, b| b.cmp(a));
    
    sizes[0] * sizes[1] * sizes[2]
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

