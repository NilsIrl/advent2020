#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<Vec<Vec<char>>>) -> usize {
    input
        .iter()
        .map(|group| {
            let mut questions = [0; 26];
            for person in group {
                for question in person {
                    questions[u32::from(*question) as usize - u32::from('a') as usize] += 1;
                }
            }
            questions.iter().filter(|question| **question != 0).count()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &Vec<Vec<Vec<char>>>) -> usize {
    input
        .iter()
        .map(|group| {
            let mut questions = [0; 26];
            for person in group {
                for question in person {
                    questions[u32::from(*question) as usize - u32::from('a') as usize] += 1;
                }
            }
            questions
                .iter()
                .filter(|question| **question == group.len())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            part1(&parse(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            11
        )
    }
}
