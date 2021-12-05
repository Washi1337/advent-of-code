use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

/// The width of the diagram.
const DIAGRAM_WIDTH: usize = 1000;

/// The height of the diagram.
const DIAGRAM_HEIGHT: usize = 1000;

/// Represents the input for the puzzle.
pub struct Input {
    /// Contains all the line segments in the puzzle instance.
    lines: Vec<LineSegment>,
}

/// Represents an xy-coordinate within a diagram.
#[derive(PartialEq, PartialOrd)]
pub struct Point(usize, usize);

/// Represents a line within a diagram.
pub struct LineSegment {
    /// The starting point. The X component of this coordinate is guaranteed
    /// to be smaller than the end point.
    pub start: Point,

    /// The ending point. The X component of this coordinate is guaranteed
    /// to be larger than the start point.
    pub end: Point,
}

/// Represents a diagram in which line segments are drawn.
pub struct Diagram {
    /// Gets the raw data stored in the diagram.
    grid: [u8; DIAGRAM_WIDTH * DIAGRAM_HEIGHT],
}

impl Point {
    /// Parses an XY coordinate from a string slice. The string must be in the format "x,y".
    pub fn from_str(s: &str) -> Point {
        let mut split = s.split(',');

        let x = split
            .next()
            .expect("Expected an X component.")
            .parse::<usize>()
            .expect("Could not parse X component.");
        let y = split
            .next()
            .expect("Expected an Y component.")
            .parse::<usize>()
            .expect("Could not parse Y component.");

        Point(x, y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.0, self.1)
    }
}

impl LineSegment {
    /// Parses a line segment from a string slice. The string must be in the format `"x1,y1 -> x2,y2"`.
    /// This function ensures that the starting point of the line segment is never to the right of
    /// the ending point.
    pub fn from_str(s: &str) -> LineSegment {
        let mut split = s.split(" -> ");

        let start = Point::from_str(split.next().expect("Expected start location."));
        let end = Point::from_str(split.next().expect("Expected end location."));

        if start < end {
            LineSegment { start, end }
        } else {
            LineSegment {
                start: end,
                end: start,
            }
        }
    }

    /// Determines whether the line segment is a horizontal line. That is, the Y coordinate does not change.
    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    /// Determines whether the line segment is a vertical line. That is, the X coordinate does not change.
    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    /// Determines whether the line segment is a diagonal line going down in the diagram.
    pub fn is_diagonal_down(&self) -> bool {
        self.start.1 < self.end.1
    }

    /// Determines whether the line segment is a diagonal line going up in the diagram.
    pub fn is_diagonal_up(&self) -> bool {
        self.start.1 > self.end.1
    }

    /// Draws the line segment in the provided diagram, and returns the number of times the line
    /// has introduced a new crossing point.
    pub fn cover(&self, diagram: &mut Diagram) -> usize {
        if self.is_horizontal() {
            (self.start.0..=self.end.0)
                .filter(|&x| diagram.cover(Point(x, self.start.1)))
                .count()
        } else if self.is_vertical() {
            (self.start.1..=self.end.1)
                .filter(|&y| diagram.cover(Point(self.start.0, y)))
                .count()
        } else {
            let length = self.end.0 - self.start.0;
            (0..=length)
                .filter(|&i| {
                    let point = Point(
                        self.start.0 + i,
                        if self.is_diagonal_down() {
                            self.start.1 + i
                        } else {
                            self.start.1 - i
                        },
                    );
                    diagram.cover(point)
                })
                .count()
        }
    }
}

impl Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl Diagram {
    /// Initializes a new empty diagram.
    pub fn new() -> Diagram {
        Diagram {
            grid: [0u8; DIAGRAM_WIDTH * DIAGRAM_HEIGHT],
        }
    }

    /// Gets the number stored at the provided coordinates.
    pub fn get(&self, location: Point) -> u8 {
        self.grid[location.1 * DIAGRAM_WIDTH + location.0]
    }

    /// Increases the number at the provided coordinates, and returns `true` if it is a new crossing point.
    pub fn cover(&mut self, location: Point) -> bool {
        let x = &mut self.grid[location.1 * DIAGRAM_HEIGHT + location.0];
        *x += 1;
        *x == 2
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DIAGRAM_HEIGHT {
            for x in 0..DIAGRAM_WIDTH {
                let cell = self.get(Point(x, y));
                if cell == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let file = File::open(file)?;

    let lines: Vec<LineSegment> = BufReader::new(file)
        .lines()
        .map(|s| LineSegment::from_str(s.expect("Expected line").as_str()))
        .collect();

    Ok(Input { lines })
}

pub fn part1(input: &Input) -> usize {
    let mut diagram = Diagram::new();

    input
        .lines
        .iter()
        .filter(|&x| x.is_horizontal() || x.is_vertical())
        .map(|l| l.cover(&mut diagram))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut diagram = Diagram::new();

    input.lines.iter().map(|l| l.cover(&mut diagram)).sum()
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

// Parse: (time: 181us)
// Solution 1: 6007 (time: 835us)
// Solution 2: 19349 (time: 938us)
