struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn new(pass: &str) -> Self {
        let (row, column) = pass.split_at(7);

        Seat {
            row: u8::from_str_radix(&row.replace("B", "1").replace("F", "0"), 2).unwrap(),
            column: u8::from_str_radix(&column.replace("R", "1").replace("L", "0"), 2).unwrap(),
        }
    }

    fn seat_id(&self) -> u32 {
        self.row as u32 * 8 + self.column as u32
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<Seat> {
    input.lines().map(|line| Seat::new(line)).collect()
}

#[aoc(day5, part1)]
fn part1(input: &Vec<Seat>) -> u32 {
    input.iter().map(Seat::seat_id).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Vec<Seat>) -> u32 {
    let mut ids: Vec<u32> = input.iter().map(Seat::seat_id).collect();
    ids.sort();
    for i in 0..ids.len() {
        if ids[i + 1] != ids[i] + 1 {
            return ids[i] + 1;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_id() {
        assert_eq!(Seat { row: 70, column: 7 }.seat_id(), 567);
        assert_eq!(Seat { row: 14, column: 7 }.seat_id(), 119);
        assert_eq!(
            Seat {
                row: 102,
                column: 4
            }
            .seat_id(),
            820
        );
    }

    #[test]
    fn seat_new() {
        let seat = Seat::new("BFFFBBFRRR");
        assert_eq!(seat.row, 70);
        assert_eq!(seat.column, 7);
        let seat = Seat::new("FFFBBBFRRR");
        assert_eq!(seat.row, 14);
        assert_eq!(seat.column, 7);
        let seat = Seat::new("BBFFBBFRLL");
        assert_eq!(seat.row, 102);
        assert_eq!(seat.column, 4);
    }
}
