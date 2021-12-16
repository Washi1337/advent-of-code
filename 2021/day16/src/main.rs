use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

/// The puzzle input.
pub struct Input {
    data: Vec<u8>,
}

/// A structure that reads individual bits from a byte stream.
pub struct BitReader<'a> {
    /// The raw data.
    pub data: &'a [u8],

    /// The current bit index.
    pub position: usize,
}

/// Errors that can occur during the reading and evaluation of a packet.
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Indicates an incorrect amount of bits was specified for reading.
    InvalidBitCount(usize),

    /// Indicates the end-of-file was encountered.
    Eof,

    /// Indicates a packet had an invalid type ID.
    InvalidTypeId(u16),
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    fn hex_value(c: u8) -> u8 {
        match c {
            b'A'..=b'F' => c - b'A' + 10,
            b'a'..=b'f' => c - b'a' + 10,
            b'0'..=b'9' => c - b'0',
            _ => panic!("Non hexadecimal digit."),
        }
    }

    let file = File::open(file)?;
    let data: Vec<u8> = BufReader::new(file)
        .lines()
        .next()
        .expect("Expected a line.")
        .unwrap()
        .as_bytes()
        .chunks(2)
        .map(|pair| (hex_value(pair[0]) << 4) | hex_value(pair[1]))
        .collect();

    Ok(Input { data })
}

impl<'a> BitReader<'a> {
    /// Creates a new bit reader at the start of the provided data buffer.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// Consumes the specified amount of bits from the input stream.
    pub fn read_bits(&mut self, mut count: usize) -> Result<u16> {
        if count > 16 {
            return Err(Error::InvalidBitCount(count));
        } else if self.position + count > 8 * self.data.len() {
            return Err(Error::Eof);
        }

        let mut result = 0u16;

        while count > 0 {
            // Calculate where we are in the buffer.
            let byte_index = self.position / 8;
            let bit_index = self.position % 8;

            // Determine how we should read the bits from the current byte.
            let chunk_width = std::cmp::min(8 - bit_index, count);
            let chunk_mask = ((1usize << chunk_width) - 1) as u8;
            let shift_count = 8 - bit_index - chunk_width;

            // Read the bits.
            let bits = (self.data[byte_index] >> shift_count) & chunk_mask;

            // Append to result.
            result <<= chunk_width;
            result |= bits as u16;

            // Advance.
            self.position += chunk_width;
            count -= chunk_width;
        }

        Ok(result)
    }

    /// Consumes a compressed literal value from the input stream.
    pub fn read_compressed_literal(&mut self) -> Result<usize> {
        let mut result = 0usize;

        loop {
            let chunk = self.read_bits(5)?;
            result <<= 4;
            result |= (chunk & 0b1111) as usize;
            if chunk & 0b10000 == 0 {
                break;
            }
        }

        Ok(result)
    }
}

pub const TYPE_ID_SUM: u16 = 0;
pub const TYPE_ID_PRODUCT: u16 = 1;
pub const TYPE_ID_MIN: u16 = 2;
pub const TYPE_ID_MAX: u16 = 3;
pub const TYPE_ID_LITERAL: u16 = 4;
pub const TYPE_ID_GT: u16 = 5;
pub const TYPE_ID_LT: u16 = 6;
pub const TYPE_ID_EQ: u16 = 7;

pub const LENGTH_TYPE_ID_BIT_COUNT: u16 = 0;
pub const LENGTH_TYPE_ID_PACKET_COUNT: u16 = 1;

pub fn part1(input: &Input) -> Result<usize> {
    fn read_packet(mut reader: &mut BitReader) -> Result<usize> {
        let mut version = reader.read_bits(3)? as usize;
        let type_id = reader.read_bits(3)?;

        if type_id == TYPE_ID_LITERAL {
            // Literal, just return the version.
            let _literal = reader.read_compressed_literal()?;
            Ok(version)
        } else {
            let length_type_id = reader.read_bits(1)?;

            // Read arguments and sum their versions.
            if length_type_id == LENGTH_TYPE_ID_BIT_COUNT {
                let total_bit_length = reader.read_bits(15)? as usize;
                let end_index = reader.position + total_bit_length;

                while reader.position < end_index {
                    version += read_packet(&mut reader)?;
                }
            } else {
                let operand_count = reader.read_bits(11)? as usize;
                for _ in 0..operand_count {
                    version += read_packet(&mut reader)?;
                }
            }

            Ok(version)
        }
    }

    let mut reader = BitReader::new(input.data.as_slice());
    read_packet(&mut reader)
}

pub fn part2(input: &Input) -> Result<usize> {
    fn evaluate(mut reader: &mut BitReader, mut eval_stack: &mut Vec<usize>) -> Result<usize> {
        let _version = reader.read_bits(3)? as usize;
        let type_id = reader.read_bits(3)?;

        if type_id == TYPE_ID_LITERAL {
            // Literal, just return the result.
            Ok(reader.read_compressed_literal()?)
        } else {
            let length_type_id = reader.read_bits(1)?;
            let mut operand_count = 0;

            // Read operands and push onto the eval stack.
            if length_type_id == LENGTH_TYPE_ID_BIT_COUNT {
                let total_bit_length = reader.read_bits(15)? as usize;
                let end_index = reader.position + total_bit_length;

                while reader.position < end_index {
                    // Recursively evaluate child packet.
                    let result = evaluate(&mut reader, &mut eval_stack)?;
                    eval_stack.push(result);
                    operand_count += 1;
                }
            } else {
                operand_count = reader.read_bits(11)? as usize;

                for _ in 0..operand_count {
                    // Recursively evaluate child packet.
                    let result = evaluate(&mut reader, &mut eval_stack)?;
                    eval_stack.push(result);
                }
            }

            // Slice out operands.
            let operands = &eval_stack[eval_stack.len() - operand_count..];

            // Compute result based on operation.
            let result = match type_id {
                TYPE_ID_SUM => Ok(operands.iter().sum::<usize>()),
                TYPE_ID_PRODUCT => Ok(operands.iter().product::<usize>()),
                TYPE_ID_MIN => Ok(*operands.iter().min().unwrap()),
                TYPE_ID_MAX => Ok(*operands.iter().max().unwrap()),
                TYPE_ID_GT => Ok((operands[0] > operands[1]) as usize),
                TYPE_ID_LT => Ok((operands[0] < operands[1]) as usize),
                TYPE_ID_EQ => Ok((operands[0] == operands[1]) as usize),
                _ => Err(Error::InvalidTypeId(type_id)),
            };

            // Pop operands from stack.
            eval_stack.resize(eval_stack.len() - operand_count, 0);

            // Return result.
            result
        }
    }

    let mut reader = BitReader::new(input.data.as_slice());
    let mut eval_stack = Vec::with_capacity(128);
    evaluate(&mut reader, &mut eval_stack)
}

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    let input = parse_input("input.txt")?;
    let time_parse = now.elapsed();
    println!("Parse: (time: {}us)", time_parse.as_micros());

    let now = Instant::now();
    let result1 = part1(&input).unwrap();
    let time1 = now.elapsed();
    println!("Solution 1: {} (time: {}us)", result1, time1.as_micros());

    let now = Instant::now();
    let result2 = part2(&input).unwrap();
    let time2 = now.elapsed();
    println!("Solution 2: {} (time: {}us)", result2, time2.as_micros());

    Ok(())
}

// Parse: (time: 80us)
// Solution 1: 897 (time: 10us)
// Solution 2: 9485076995911 (time: 11us)

// part 1 (real)           time:   [4.4253 us 4.4294 us 4.4338 us]
// part 2 (real)           time:   [4.6803 us 4.6849 us 4.6900 us]
