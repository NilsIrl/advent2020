use std::io::BufRead;

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter(|line| {
                let mut split = line.as_ref().unwrap().split(' ');
                let (min, max): (usize, usize) = {
                    let mut range = split.next().unwrap().split('-');
                    (
                        range.next().unwrap().parse().unwrap(),
                        range.next().unwrap().parse().unwrap(),
                    )
                };
                let c = split.next().unwrap().chars().next().unwrap() as u8;
                let password = split.next().unwrap().as_bytes();
                (password[min - 1] == c) != (password[max - 1] == c)
            })
            .count()
    );
}
