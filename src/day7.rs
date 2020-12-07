use std::collections::{HashMap, HashSet};

#[aoc_generator(day7, part1)]
fn generator(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut target = String::new();
        let mut words = line.split_ascii_whitespace();
        target.push_str(words.next().unwrap());
        target.push(' ');
        target.push_str(words.next().unwrap());
        let (_, list) = line.split_once(" contain ").unwrap();
        for bag in list.split(',') {
            let bag = bag.trim_matches(|c| c == ' ' || c == '.');
            if bag == "no other bags" {
                break;
            }
            let mut bag_words = bag.split_ascii_whitespace();
            bag_words.next().unwrap();
            let mut source = String::new();
            source.push_str(bag_words.next().unwrap());
            source.push(' ');
            source.push_str(bag_words.next().unwrap());
            map.entry(source).or_insert(Vec::new()).push(target.clone());
        }
    }
    map
}

fn add(set: &mut HashSet<String>, map: &HashMap<String, Vec<String>>, needle: &str) {
    if let Some(vec) = map.get(needle) {
        for x in vec.iter() {
            set.insert(x.to_string());
            add(set, map, x);
        }
    }
}

#[aoc(day7, part1)]
fn part1(map: &HashMap<String, Vec<String>>) -> usize {
    let mut set = HashSet::new();
    add(&mut set, map, "shiny gold");
    set.len()
}

#[aoc_generator(day7, part2)]
fn generator_part2(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut target = String::new();
        let mut words = line.split_ascii_whitespace();
        target.push_str(words.next().unwrap());
        target.push(' ');
        target.push_str(words.next().unwrap());
        map.insert(target.clone(), Vec::new());
        let (_, list) = line.split_once(" contain ").unwrap();
        for bag in list.split(',') {
            let bag = bag.trim_matches(|c| c == ' ' || c == '.');
            if bag == "no other bags" {
                break;
            }
            let mut bag_words = bag.split_ascii_whitespace();
            let count = bag_words.next().unwrap().parse().unwrap();
            let mut source = String::new();
            source.push_str(bag_words.next().unwrap());
            source.push(' ');
            source.push_str(bag_words.next().unwrap());
            map.get_mut(&target).unwrap().push((count, source));
        }
    }
    map
}

fn bag_inside(map: &HashMap<String, Vec<(usize, String)>>, needle: &str) -> usize {
    let mut sum = 0;
    for (count, bag) in map[needle].iter() {
        sum += count;
        sum += count * bag_inside(map, &bag);
    }
    sum
}

#[aoc(day7, part2)]
fn part2(map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    bag_inside(map, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(
            part2(&generator_part2(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
            )),
            32
        );
    }
}
