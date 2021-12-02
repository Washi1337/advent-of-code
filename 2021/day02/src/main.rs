use std::{fs::File, io::{BufReader, BufRead}, time::Instant};


#[derive(Debug)]
enum Direction {
    Forward, 
    Down,
    Up
}


#[derive(Debug)]
struct Move {
    pub direction: Direction,
    pub distance: usize
}


#[derive(Debug)]
struct Vector2(usize, usize);


#[derive(Debug)]
struct Vector3(usize, usize, usize);


impl Move {
    pub fn from_str(s: &str) -> Option<Self> {
        let mut split = s.split(' ');

        let direction = match split.next()? {
            "forward" => Some(Direction::Forward),
            "down"    => Some(Direction::Down),
            "up"      => Some(Direction::Up),
            _         => None
        }?;

        let distance = split.next()?
            .parse::<usize>()
            .ok()?;

        Some(Self { direction, distance })
    }

    pub fn traverse1(&self, pos: Vector2) -> Vector2 {
        match self.direction {
            Direction::Forward => Vector2(pos.0 + self.distance, pos.1),
            Direction::Down    => Vector2(pos.0, pos.1 + self.distance),
            Direction::Up      => Vector2(pos.0, pos.1 - self.distance)
        }
    }

    pub fn traverse2(&self, pos: Vector3) -> Vector3 {
        match self.direction {
            Direction::Forward => Vector3(pos.0 + self.distance, pos.1 + self.distance * pos.2, pos.2),
            Direction::Down    => Vector3(pos.0, pos.1, pos.2 + self.distance),
            Direction::Up      => Vector3(pos.0, pos.1, pos.2 - self.distance)
        }
    }
}


fn part1(input: &Vec<Move>) -> usize {
    let start = Vector2(0, 0);
    let end = input
        .iter()
        .fold(start, |acc, x| x.traverse1(acc));
    end.0 * end.1
}


fn part2(input: &Vec<Move>) -> usize {
    let start = Vector3(0, 0, 0);
    let end = input
        .iter()
        .fold(start, |acc, x| x.traverse2(acc));
    end.0 * end.1
}


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let input: Vec<Move> = BufReader::new(file)
        .lines()
        .map(|x| Move::from_str(x.unwrap().as_str()).unwrap())
        .collect();
    
    let now = Instant::now();
    let result1 = part1(&input);
    let elapsed1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&input);
    let elapsed2 = now.elapsed();

    println!("Part1: {} (time: {})", result1, elapsed1.as_nanos());
    println!("Part2: {} (time: {})", result2, elapsed2.as_nanos());
    Ok(())
}
