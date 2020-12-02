use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut set: HashSet<u64> = HashSet::new();
    for n in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
    {
        set.insert(n);
    }

    for x in set.iter() {
        if set.contains(&(2020 - x)) {
            println!("{}", x * (2020 - x));
        }
    }
}
