use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;

#[aoc(day1, part1)]
fn part1(input: &str) -> u64 {
    let set: HashSet<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    for x in set.iter() {
        if set.contains(&(2020 - x)) {
            return x * (2020 - x);
        }
    }
    unreachable!();
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u64 {
    let vec: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    for c in vec.iter().combinations(3) {
        if c.iter().map(|x| *x).sum::<u64>() == 2020 {
            return c.iter().map(|x| *x).product::<u64>();
        }
    }
    unreachable!();
}
