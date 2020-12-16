use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

struct Data {
    fields: HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
    ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
fn generator(input: &str) -> Data {
    lazy_static::lazy_static! {
        static ref FIELD_RE: Regex =
            Regex::new(r"\A(.+): (\d+)-(\d+) or (\d+)-(\d+)\z").unwrap();
    }
    let mut blocks = input.split("\n\n");
    let fields = blocks
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let caps = FIELD_RE.captures(line).unwrap();
            (
                caps[1].to_string(),
                (
                    caps[2].parse().unwrap()..=caps[3].parse().unwrap(),
                    caps[4].parse().unwrap()..=caps[5].parse().unwrap(),
                ),
            )
        })
        .collect();
    let ticket = blocks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby_tickets = blocks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Data {
        fields,
        ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn part1(input: &Data) -> usize {
    let mut sum = 0;
    for ticket in input.nearby_tickets.iter() {
        for value in ticket.iter() {
            let mut work = false;
            for (range1, range2) in input.fields.values() {
                if range1.contains(value) || range2.contains(value) {
                    work = true;
                    break;
                }
            }
            if !work {
                sum += value;
            }
        }
    }
    sum
}

#[aoc(day16, part2)]
fn part2(input: &Data) -> usize {
    let nearby_tickets: Vec<&Vec<usize>> = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|value| {
                input
                    .fields
                    .values()
                    .any(|(range1, range2)| range1.contains(value) || range2.contains(value))
            })
        })
        .collect();

    let mut remaining_fields: HashSet<&String> = input.fields.keys().collect();
    let mut solutions = HashMap::new();
    while !remaining_fields.is_empty() {
        for i in 0..input.ticket.len() {
            let s: Vec<_> = remaining_fields
                .iter()
                .filter(|field| {
                    let (range1, range2) = &input.fields[**field];
                    nearby_tickets
                        .iter()
                        .all(|ticket| range1.contains(&ticket[i]) || range2.contains(&ticket[i]))
                })
                .collect();
            if s.len() == 1 {
                solutions.insert(s[0].clone(), i);
                remaining_fields.remove(s[0].clone());
                continue;
            }
        }
    }
    solutions
        .iter()
        .filter_map(|(key, val)| {
            if key.starts_with("departure") {
                Some(input.ticket[*val])
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(
            part1(&generator(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            )),
            71
        );
    }

    #[test]
    fn example_part2() {
        part2(&generator(
            "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9",
        ));
    }
}
