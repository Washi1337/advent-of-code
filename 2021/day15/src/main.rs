use std::{
    collections::BinaryHeap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Div, Mul, Rem, Sub},
    time::Instant,
};

/// A 2 dimensional integer vector. Used for positions and directions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector2(isize, isize);

/// All directions that we can go in the grid.
const DIRECTIONS: [Vector2; 4] = [Vector2(1, 0), Vector2(0, 1), Vector2(-1, 0), Vector2(0, -1)];

// Some cool operator overloading in rust, for extra internet puntos :^).

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<isize> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: isize) -> Self::Output {
        Vector2(self.0 / rhs, self.1 / rhs)
    }
}

impl Rem<isize> for Vector2 {
    type Output = Vector2;

    fn rem(self, rhs: isize) -> Self::Output {
        Vector2(self.0 % rhs, self.1 % rhs)
    }
}

/// Represents a 2 dimensional square grid.
pub struct Grid<T>
where
    T: Clone + Copy,
{
    /// The raw data in the grid.
    pub grid: Vec<T>,

    /// The size of one of the dimensions.
    pub size: isize,
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    /// Creates a new square grid with the provided initializer value.
    pub fn new(size: isize, init: T) -> Self {
        Self {
            grid: vec![init; (size * size) as usize],
            size,
        }
    }

    /// Gets an element in the grid by its position.
    pub fn get(&self, location: Vector2) -> T {
        self.grid[(location.1 * self.size + location.0) as usize]
    }

    /// Sets an element in the grid by its position.
    pub fn set(&mut self, location: Vector2, value: T) {
        self.grid[(location.1 * self.size + location.0) as usize] = value;
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "{}", self.get(Vector2(x as isize, y as isize)))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

/// The puzzle input.
pub struct Input {
    /// The input grid.
    pub grid: Grid<u8>,
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;
    let lines = BufReader::new(file).lines();

    let grid: Vec<u8> = lines
        .flat_map(|ln| {
            ln.expect("Expected a line")
                .as_bytes()
                .iter()
                .map(|b| b - '0' as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let size = (grid.len() as f64).sqrt() as isize;
    Ok(Input {
        grid: Grid { grid, size },
    })
}

/// Contains information on the current route that we are taking in the path finding algorithm.
/// We implement [`Ord`] and [`PartialOrd`] to allow storing them in a [`BinaryHeap`].
#[derive(PartialEq, Eq)]
struct RouteInfo {
    position: Vector2,
    cost: usize,
}

impl Ord for RouteInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for RouteInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the shortest path in a grid from the top-left to the bottom-right corner.
fn find_shortest_path(grid: &Grid<u8>, scale: isize) -> usize {
    let start = Vector2(0, 0);
    let end = Vector2(grid.size, grid.size) * scale - Vector2(1, 1);

    // Stores the distances table.
    let mut distances = Grid::new(grid.size * scale, usize::MAX);
    distances.set(start, 0);

    // The agenda, stored as a priority queue for fast smallest element lookups (in our case lowest distance).
    let mut agenda = BinaryHeap::with_capacity(1024);
    agenda.push(RouteInfo {
        position: start,
        cost: 0,
    });

    while let Some(current) = agenda.pop() {
        // Are we there yet?
        if current.position == end {
            return current.cost;
        }

        // Did we already find a better route for this position?
        if current.cost > distances.get(current.position) {
            continue;
        }

        // Go all possible directions.
        for direction in DIRECTIONS {
            // Get the neighbour position, and check if still in bounds.
            let neighbour = current.position + direction;
            if neighbour.0 < 0
                || neighbour.0 >= distances.size
                || neighbour.1 < 0
                || neighbour.1 >= distances.size
            {
                continue;
            }

            // Deterine tile coordinate and the original neighbour position that this neighbour is  (potentially) a repetition of.
            let tile = neighbour / grid.size;
            let reference_neighbour = neighbour % grid.size;

            // Cost to get to the neighbour is the number in the grid. Since all tiles are just repetitions of the first tile, but
            // every tile coordinate increases the cost by one, we can simply calculate the new cost quickly without storing all tiles.
            let absolute_cost = grid.get(reference_neighbour) as isize + tile.0 + tile.1;
            let normalized_cost = (absolute_cost - 1) % 9 + 1;

            // Compute total cost of our newly extended route.
            let new_total_cost = current.cost + normalized_cost as usize;

            // Is this actually a better route than we had before?
            if new_total_cost < distances.get(neighbour) {
                // Remember route, and schedule neighbour for processing.
                distances.set(neighbour, new_total_cost);
                agenda.push(RouteInfo {
                    position: neighbour,
                    cost: new_total_cost,
                });
            }
        }
    }

    distances.get(end)
}

pub fn part1(input: &Input) -> usize {
    find_shortest_path(&input.grid, 1)
}

pub fn part2(input: &Input) -> usize {
    find_shortest_path(&input.grid, 5)
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

// Parse: (time: 194us)
// Solution 1: 503 (time: 927us)
// Solution 2: 2853 (time: 24559us)

// part 1 (real)           time:   [868.96 us 872.84 us 878.01 us]
// part 2 (real)           time:   [23.824 ms 23.855 ms 23.888 ms]