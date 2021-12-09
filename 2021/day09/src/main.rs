use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 100;
const MAX_HEIGHT: u8 = 9;

/// Represents a position within a height map.
#[derive(Clone, Copy)]
pub struct Vector2(usize, usize);

/// Represents a height map in the form of u8 elements.
pub struct HeightMap {
    grid: [u8; MAP_WIDTH * MAP_HEIGHT],
}

/// Provides heights for neighbouring cells.
pub struct Neighbours {
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
}

/// Represents the input for the puzzle.
pub struct Input {
    map: HeightMap,
}

impl Vector2 {
    /// Translates the index into a position on a height map.
    pub fn from_index(index: usize) -> Self {
        Self(index % MAP_WIDTH, index / MAP_WIDTH)
    }

    /// Translates the position into an index within the raw grid of a height map.
    pub fn to_index(&self) -> usize {
        self.1 * MAP_WIDTH + self.0
    }
}

impl HeightMap {
    /// Creates a new height map, that is initialized with the max height on every cell.
    pub fn new() -> Self {
        Self {
            grid: [MAX_HEIGHT; MAP_WIDTH * MAP_HEIGHT],
        }
    }

    /// Gets the height at the provided position.
    pub fn get(&self, location: Vector2) -> u8 {
        self.grid[location.to_index()]
    }

    /// Updates the height at the provided position.
    pub fn set(&mut self, location: Vector2, height: u8) {
        self.grid[location.to_index()] = height;
    }

    /// Gets the heights of all neighbours of the provided location.
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
            left,
        }
    }

    /// Determines whether the provided location is a low point.
    pub fn is_low_point(&self, location: Vector2) -> bool {
        let height = self.get(location);
        if height == MAX_HEIGHT {
            return false;
        }

        let neighbours = self.get_neighbours(location);
        height < neighbours.top
            && height < neighbours.right
            && height < neighbours.bottom
            && height < neighbours.left
    }

    /// Computes the risk level for the provided risk level.
    pub fn get_risk_level(&self, location: Vector2) -> usize {
        (self.get(location) + 1) as usize
    }

    /// Computes the size of the basin, starting at the provided location.
    /// This location does NOT have to be a low point. It returns [`None`] if the
    /// cell was already visited or if the cell has the value [`MAX_HEIGHT`].
    pub fn get_basin_size(
        &self,
        location: Vector2,
        visited: &mut [bool],
        agenda: &mut Vec<Vector2>,
    ) -> Option<usize> {

        // Short circuit if possible.
        if visited[location.to_index()] || self.get(location) == MAX_HEIGHT {
            return None;
        }

        let mut size = 0;

        // Perform DFS.
        agenda.push(location);
        while !agenda.is_empty() {
            let location = agenda.pop().unwrap();
            let index = location.to_index();

            if visited[index] {
                continue;
            }

            visited[index] = true;
            size += 1;

            let neighbours = self.get_neighbours(location);
            if neighbours.left != MAX_HEIGHT {
                agenda.push(Vector2(location.0 - 1, location.1))
            }
            if neighbours.right != MAX_HEIGHT {
                agenda.push(Vector2(location.0 + 1, location.1))
            }
            if neighbours.top != MAX_HEIGHT {
                agenda.push(Vector2(location.0, location.1 - 1))
            }
            if neighbours.bottom != MAX_HEIGHT {
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
        line.expect("Expected height map line")
            .as_bytes()
            .iter()
            .map(|&b| b - 0x30)
            .enumerate()
            .for_each(|(x, h)| map.set(Vector2(x, y), h));
    });

    Ok(Input { map })
}

pub fn part1(input: &Input) -> usize {
    (0..MAP_HEIGHT).map(|y| {
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
    })
    .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut visited = [false; MAP_WIDTH * MAP_HEIGHT];
    let mut agenda = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
    let mut top = [0usize; 3];

    (0..MAP_WIDTH * MAP_HEIGHT)
        .filter_map(|i|
            input.map.get_basin_size(Vector2::from_index(i), &mut visited, &mut agenda)
        )
        .for_each(|size| {
            if size >= top[0] {
                top[2] = top[1];
                top[1] = top[0];
                top[0] = size;
            } else if size >= top[1] {
                top[2] = top[1];
                top[1] = size;
            } else if size > top[2] {
                top[2] = size;
            }
        });

    top.iter().product()
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
