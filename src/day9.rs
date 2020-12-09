#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<u64>) -> u64 {
    for range in 25..input.len() {
        let mut found = false;
        for first_number in input[range - 25..range].iter() {
            for second_number in input[range - 25..range].iter() {
                if second_number == first_number {
                    continue;
                }
                if first_number + second_number == input[range] {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return input[range];
        }
    }
    unreachable!();
}

#[aoc(day9, part2)]
fn part2(input: &Vec<u64>) -> u64 {
    let sum_to_reach = part1(&input);
    let mut start = 0;
    let mut end = 0;
    let mut running_sum = 0;
    loop {
        if running_sum < sum_to_reach {
            running_sum += input[end];
            end += 1;
        } else if running_sum > sum_to_reach {
            running_sum -= input[start];
            start += 1;
        } else {
            return input[start..end].iter().min().unwrap()
                + input[start..end].iter().max().unwrap();
        }
    }
}
