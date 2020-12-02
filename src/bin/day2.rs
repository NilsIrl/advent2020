use std::io::BufRead;

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter(|line| {
                let mut split = line.as_ref().unwrap().split(' ');
                let (min, max) = {
                    let mut range = split.next().unwrap().split('-');
                    (
                        range.next().unwrap().parse().unwrap(),
                        range.next().unwrap().parse().unwrap(),
                    )
                };
                let c = split.next().unwrap().chars().next().unwrap();
                let password = split.next().unwrap();
                let count = password.matches(c).count();
                count >= min && count <= max
            })
            .count()
    );
}
