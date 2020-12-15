#[derive(Clone)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl Position {
    fn new(position: char) -> Self {
        match position {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unimplemented!(),
        }
    }

    fn occupied(&self) -> bool {
        match self {
            Self::Occupied => true,
            Self::Floor | Self::Empty => false,
        }
    }
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Position::new(c)).collect())
        .collect()
}

fn count_occupied(y: i64, x: i64, map: &Vec<Vec<Position>>) -> usize {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ]
    .iter()
    .filter(|(y_diff, x_diff)| {
        let new_y = y + y_diff;
        let new_x = x + x_diff;
        if new_y >= 0 && new_x >= 0 {
            map.get(new_y as usize).map_or(false, |row| {
                row.get(new_x as usize)
                    .map_or(false, |position| position.occupied())
            })
        } else {
            false
        }
    })
    .count()
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<Position>>) -> usize {
    let mut a = (*input).clone();
    let mut b = a.clone();
    loop {
        let mut changed = false;

        for y in 0..a.len() {
            for x in 0..a[0].len() {
                match a[y][x] {
                    Position::Empty => {
                        if count_occupied(y as i64, x as i64, &a) == 0 {
                            b[y][x] = Position::Occupied;
                            changed = true;
                        } else {
                            b[y][x] = Position::Empty;
                        }
                    }
                    Position::Occupied => {
                        if count_occupied(y as i64, x as i64, &a) >= 4 {
                            b[y][x] = Position::Empty;
                            changed = true;
                        } else {
                            b[y][x] = Position::Occupied;
                        }
                    }
                    Position::Floor => (),
                }
            }
        }

        std::mem::swap(&mut a, &mut b);

        if !changed {
            return b
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|position| match position {
                            Position::Occupied => true,
                            _ => false,
                        })
                        .count()
                })
                .sum();
        }
    }
}

fn count_occupied_part2(y: i64, x: i64, map: &Vec<Vec<Position>>) -> usize {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ]
    .iter()
    .filter(|(y_diff, x_diff)| {
        let mut new_y = y;
        let mut new_x = x;

        loop {
            new_y += y_diff;
            new_x += x_diff;
            if new_y >= 0 && new_x >= 0 {
                match map
                    .get(new_y as usize)
                    .map(|row| row.get(new_x as usize))
                    .flatten()
                {
                    Some(Position::Floor) => (),
                    Some(Position::Empty) | None => return false,
                    Some(Position::Occupied) => return true,
                };
            } else {
                return false;
            }
        }
    })
    .count()
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<Position>>) -> usize {
    let mut a = (*input).clone();
    let mut b = a.clone();
    loop {
        let mut changed = false;

        for y in 0..a.len() {
            for x in 0..a[0].len() {
                match a[y][x] {
                    Position::Empty => {
                        if count_occupied_part2(y as i64, x as i64, &a) == 0 {
                            b[y][x] = Position::Occupied;
                            changed = true;
                        } else {
                            b[y][x] = Position::Empty;
                        }
                    }
                    Position::Occupied => {
                        if count_occupied_part2(y as i64, x as i64, &a) >= 5 {
                            b[y][x] = Position::Empty;
                            changed = true;
                        } else {
                            b[y][x] = Position::Occupied;
                        }
                    }
                    Position::Floor => (),
                }
            }
        }

        std::mem::swap(&mut a, &mut b);

        if !changed {
            return b
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|position| match position {
                            Position::Occupied => true,
                            _ => false,
                        })
                        .count()
                })
                .sum();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = generator(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );

        assert_eq!(part1(&input), 37);
        assert_eq!(part2(&input), 26);
    }
}
