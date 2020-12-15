enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn from_str(input: &str) -> Self {
        match input {
            "N" => Self::North,
            "S" => Self::South,
            "E" => Self::East,
            "W" => Self::West,
            "L" => Self::Left,
            "R" => Self::Right,
            "F" => Self::Forward,
            _ => unimplemented!(),
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        for _ in 0..(if let Self::Left = instruction.direction {
            4 - instruction.magnitude / 90
        } else {
            instruction.magnitude / 90
        }) {
            *self = match self {
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
                Self::North => Self::East,
                _ => unimplemented!(),
            };
        }
    }
}

struct Instruction {
    direction: Direction,
    magnitude: i64,
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (action, value) = line.split_at(1);
            Instruction {
                direction: Direction::from_str(action),
                magnitude: value.parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Instruction>) -> i64 {
    let mut direction = Direction::East;
    let mut east = 0;
    let mut north = 0;

    for x in input.iter() {
        match x.direction {
            Direction::North => north += x.magnitude,
            Direction::South => north -= x.magnitude,
            Direction::East => east += x.magnitude,
            Direction::West => east -= x.magnitude,
            Direction::Left | Direction::Right => direction.rotate(x),
            Direction::Forward => match direction {
                Direction::North => north += x.magnitude,
                Direction::South => north -= x.magnitude,
                Direction::East => east += x.magnitude,
                Direction::West => east -= x.magnitude,
                _ => unreachable!(),
            },
        }
    }

    east.abs() + north.abs()
}

fn rotate_around_origin_right(way_east: &mut i64, way_north: &mut i64) {
    (*way_east, *way_north) = (*way_north, -*way_east)
}

#[aoc(day12, part2)]
fn part2(input: &Vec<Instruction>) -> i64 {
    let mut east = 0;
    let mut north = 0;
    let mut way_east = 10;
    let mut way_north = 1;

    for x in input.iter() {
        match x.direction {
            Direction::North => way_north += x.magnitude,
            Direction::South => way_north -= x.magnitude,
            Direction::East => way_east += x.magnitude,
            Direction::West => way_east -= x.magnitude,
            Direction::Left => {
                for _ in 0..(4 - x.magnitude / 90) {
                    rotate_around_origin_right(&mut way_east, &mut way_north);
                }
            }
            Direction::Right => {
                for _ in 0..(x.magnitude / 90) {
                    rotate_around_origin_right(&mut way_east, &mut way_north);
                }
            }
            Direction::Forward => {
                east += way_east * x.magnitude;
                north += way_north * x.magnitude;
            }
        }
    }

    east.abs() + north.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example1_input = "F10
N3
F7
R90
F11";
        assert_eq!(part1(&generator(example1_input)), 25);
        assert_eq!(part2(&generator(example1_input)), 286);
    }

    #[test]
    fn test_rotate_around_origin_right() {
        let mut way_east = 5;
        let mut way_north = 0;
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, 0);
        assert_eq!(way_north, -5);
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, -5);
        assert_eq!(way_north, 0);
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, 0);
        assert_eq!(way_north, 5);
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, 5);
        assert_eq!(way_north, 0);
    }

    #[test]
    fn test_rotate_more_complex() {
        let mut way_east = 3;
        let mut way_north = 2;
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, 2);
        assert_eq!(way_north, -3);
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, -3);
        assert_eq!(way_north, -2);
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, -2);
        assert_eq!(way_north, 3);
        rotate_around_origin_right(&mut way_east, &mut way_north);
        assert_eq!(way_east, 3);
        assert_eq!(way_north, 2);
    }
}
