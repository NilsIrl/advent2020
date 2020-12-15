use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn parse_assignment(line: &str) -> (u64, u64) {
    lazy_static! {
        static ref ASSIGNMENT_RE: Regex = Regex::new(r"\Amem\[(\d+)\] = (\d+)\z").unwrap();
    }
    let caps = ASSIGNMENT_RE.captures(line).unwrap();
    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut map = HashMap::new();
    let mut mask = "";

    for line in input.lines() {
        if line.starts_with("mask") {
            mask = &line[7..];
        } else {
            let (index, mut value) = parse_assignment(line);
            for (i, bit) in mask.chars().rev().enumerate() {
                match bit {
                    '0' => value &= !(1 << i),
                    '1' => value |= 1 << i,
                    'X' => (),
                    _ => unreachable!(),
                }
            }
            map.insert(index, value);
        }
    }

    map.values().sum()
}

#[aoc(day14, part1, vec)]
fn part1_vec(input: &str) -> u64 {
    let mut map = HashMap::new();
    let mut zero_indices = Vec::with_capacity(36);
    let mut one_indices = Vec::with_capacity(36);

    for line in input.lines() {
        if line.starts_with("mask") {
            zero_indices.clear();
            one_indices.clear();
            for (i, bit) in line[7..].chars().rev().enumerate() {
                match bit {
                    '0' => zero_indices.push(i),
                    '1' => one_indices.push(i),
                    _ => (),
                }
            }
        } else {
            let (index, mut value) = parse_assignment(line);
            for zero_i in zero_indices.iter() {
                value &= !(1 << zero_i);
            }
            for one_i in one_indices.iter() {
                value |= 1 << one_i
            }
            map.insert(index, value);
        }
    }

    map.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut map = HashMap::new();
    let mut zero_indices = Vec::with_capacity(36);
    let mut one_indices = Vec::with_capacity(36);
    let mut floating_indices = Vec::with_capacity(36);

    for line in input.lines() {
        if line.starts_with("mask") {
            zero_indices.clear();
            one_indices.clear();
            floating_indices.clear();
            for (i, bit) in line[7..].chars().rev().enumerate() {
                match bit {
                    '0' => zero_indices.push(i),
                    '1' => one_indices.push(i),
                    'X' => floating_indices.push(i),
                    _ => unreachable!(),
                }
            }
        } else {
            let (mut index, value) = parse_assignment(line);
            for one_i in one_indices.iter() {
                index |= 1 << one_i;
            }
            for count in 0..2usize.pow(floating_indices.len() as u32) {
                for (i, val) in floating_indices.iter().enumerate() {
                    if count & 1 << i != 0 {
                        // bit needs to be set
                        index |= 1 << val;
                    } else {
                        // bit needs to be cleared
                        index &= !(1 << val);
                    }
                }
                map.insert(index, value);
            }
        }
    }

    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            part1(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            ),
            165
        )
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            part2(
                "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            ),
            208
        );
    }
}
