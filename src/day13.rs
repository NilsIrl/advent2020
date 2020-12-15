#[aoc(day13, part1)]
fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let time: u64 = lines.next().unwrap().parse().unwrap();
    let (a, b) = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .map(|id: u64| (id, id - time % id))
        .min_by_key(|(_, wait)| *wait)
        .unwrap();
    a * b
}

#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
    let mut values: Vec<(usize, usize)> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, val)| {
            if val == "x" {
                None
            } else {
                Some((i, val.parse().unwrap()))
            }
        })
        .collect();
    values.sort_by_key(|k| k.1);
    values.reverse();
    for i in 1.. {
        let t = (values[0].1 * i) - values[0].0;
        let mut works = true;
        for x in values.iter().skip(1) {
            if (t + x.0) % x.1 != 0 {
                works = false;
                break;
            }
        }
        if works {
            return t;
        }
        if i % 10_000_000 == 0 {
            dbg!(t);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(
            part1(
                "939
7,13,x,x,59,x,31,19"
            ),
            295
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2("a\n7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(part2("a\n67,7,59,61"), 754018);
        assert_eq!(part2("a\n67,x,7,59,61"), 779210);
        assert_eq!(part2("a\n67,7,x,59,61"), 1261476);
        assert_eq!(part2("a\n1789,37,47,1889"), 1202161486);
    }
}
