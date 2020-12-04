use std::collections::HashMap;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

/*
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<HashMap<&str, &str>> {
}
*/

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let passports: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect()
        })
        .collect();
    passports
        .iter()
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|field| passport.contains_key(field))
        })
        .count()
}
