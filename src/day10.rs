use std::collections::HashMap;

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<u64>) -> u64 {
    let mut input = (*input).clone();
    input.sort();
    let mut one_jolt_differences = 0;
    let mut three_jolt_differences = 0;

    let mut current_joltage = 0;

    for i in input.iter() {
        if i - current_joltage == 1 {
            one_jolt_differences += 1;
        }
        if i - current_joltage == 3 {
            three_jolt_differences += 1;
        }

        current_joltage = *i;
    }

    one_jolt_differences * (three_jolt_differences + 1)
}

fn count(index: usize, memoization: &mut HashMap<usize, u64>, input: &Vec<u64>) -> u64 {
    if let Some(saved) = memoization.get(&index) {
        return *saved;
    }

    let mut sum = 0;

    if index + 1 < input.len() {
        if input[index + 1] - input[index] <= 3 {
            sum += count(index + 1, memoization, &input);
        }
    }

    if index + 2 < input.len() {
        if input[index + 2] - input[index] <= 3 {
            sum += count(index + 2, memoization, &input);
        }
    }

    if index + 3 < input.len() {
        if input[index + 3] - input[index] <= 3 {
            sum += count(index + 3, memoization, &input);
        }
    }

    memoization.insert(index, std::cmp::max(sum, 1));
    *memoization.get(&index).unwrap()
}

#[aoc(day10, part2)]
fn part2(input: &Vec<u64>) -> u64 {
    let mut input = (*input).clone();
    input.push(0);
    input.sort();

    let mut memoization = HashMap::new();

    count(0, &mut memoization, &input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            part1(&generator(
                "16
10
15
5
1
11
7
19
6
12
4"
            )),
            5 * 7
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(
            part2(&generator(
                "16
10
15
5
1
11
7
19
6
12
4"
            )),
            8
        );
    }
}
