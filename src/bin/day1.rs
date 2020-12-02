use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut set: HashSet<u64> = HashSet::new();
    let mut vec = Vec::new();
    for n in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
    {
        set.insert(n);
        vec.push(n);
    }

    for x in set.iter() {
        if set.contains(&(2020 - x)) {
            println!("{}", x * (2020 - x));
        }
    }

    for c in vec.iter().combinations(3) {
        if c.iter().map(|x| *x).sum::<u64>() == 2020 {
            println!("{}", c.iter().map(|x| *x).product::<u64>());
        }
    }
}
