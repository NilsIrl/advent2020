enum Square {
    Tree,
    Open,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { Square::Open } else { Square::Tree })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Vec<Square>>) -> u64 {
    let mut x = 0;
    let mut trees = 0;
    for row in input.iter() {
        match row[x % row.len()] {
            Square::Tree => trees += 1,
            Square::Open => (),
        };
        x += 3;
    }
    return trees;
}

#[aoc(day3, part1, func)]
fn part1_func(input: &Vec<Vec<Square>>) -> u64 {
    num_tree(3, 1, input)
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Vec<Square>>) -> u64 {
    num_tree(1, 1, input)
        * num_tree(3, 1, input)
        * num_tree(5, 1, input)
        * num_tree(7, 1, input)
        * num_tree(1, 2, input)
}

fn num_tree(right: usize, down: usize, input: &Vec<Vec<Square>>) -> u64 {
    let mut x = 0;
    let mut trees = 0;
    for row in input.iter().step_by(down) {
        match row[x % row.len()] {
            Square::Tree => trees += 1,
            Square::Open => (),
        };
        x += right;
    }
    return trees;
}
