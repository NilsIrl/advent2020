use std::collections::HashMap;

#[aoc_generator(day15)]
fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn solve(input: &[usize], iter: usize) -> usize {
    let mut indices: HashMap<usize, usize> = HashMap::new();
    for (i, x) in input.iter().enumerate() {
        indices.insert(*x, i);
    }
    let mut last_value = input[input.len() - 1];
    for i in (input.len() - 1)..(iter - 1) {
        last_value = indices.insert(last_value, i).map_or(0, |index| i - index);
    }
    last_value
}

#[aoc(day15, part1)]
fn part1(input: &[usize]) -> usize {
    solve(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &[usize]) -> usize {
    solve(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(&[1, 3, 2]), 1);
        assert_eq!(part1(&[2, 1, 3]), 10);
        assert_eq!(part1(&[1, 2, 3]), 27);
        assert_eq!(part1(&[2, 3, 1]), 78);
        assert_eq!(part1(&[3, 2, 1]), 438);
        assert_eq!(part1(&[3, 1, 2]), 1836);
    }
}
