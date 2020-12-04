use std::collections::HashMap;

fn parse(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let passports = parse(&input);
    passports
        .iter()
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|field| passport.contains_key(field))
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    const REQUIRED_FIELDS: [(&str, &dyn Fn(&str) -> bool); 7] = [
        ("byr", &|x: &str| {
            (1920..=2002).contains(&x.parse().unwrap())
        }),
        ("iyr", &|x: &str| {
            (2010..=2020).contains(&x.parse().unwrap())
        }),
        ("eyr", &|x: &str| {
            (2020..=2030).contains(&x.parse().unwrap())
        }),
        ("hgt", &|x| {
            x.find(|c: char| c.is_ascii_alphabetic())
                .map_or(false, |mid| {
                    let (val, unit) = x.split_at(mid);
                    let val: i32 = match val.parse() {
                        Ok(val) => val,
                        _ => return false,
                    };
                    match unit {
                        "cm" => (150..=193).contains(&val),
                        "in" => (59..=76).contains(&val),
                        _ => false,
                    }
                })
        }),
        ("hcl", &|x| {
            if x.len() != 7 {
                return false;
            }
            let mut chars = x.chars();
            match chars.next() {
                Some('#') => chars.all(|c| {
                    if c.is_ascii_hexdigit() {
                        if c.is_ascii_alphabetic() {
                            c.is_lowercase()
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }),
                a => false,
            }
        }),
        ("ecl", &|x| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x)
        }),
        ("pid", &|x: &str| {
            x.len() == 9 && x.chars().all(|c| c.is_ascii_digit())
        }),
    ];
    let passports = parse(&input);
    passports
        .iter()
        .filter(|passport| {
            REQUIRED_FIELDS.iter().all(|(field, predicate)| {
                passport.get(field).map_or(false, |value| {
                    let ok = predicate(value);
                    ok
                })
            })
        })
        .count()
}
