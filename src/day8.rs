use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<(Operation, i64)> {
    input
        .lines()
        .map(|line| {
            let (opcode, argument) = line.split_once(' ').unwrap();
            (
                match opcode {
                    "acc" => Operation::Acc,
                    "jmp" => Operation::Jmp,
                    "nop" => Operation::Nop,
                    _ => unreachable!(),
                },
                argument.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(program: &Vec<(Operation, i64)>) -> i64 {
    let mut accumulator = 0;
    let mut ip: i64 = 0;
    let mut visited_instructions = HashSet::new();
    while !visited_instructions.contains(&ip) {
        visited_instructions.insert(ip);
        let instruction = &program[ip as usize];
        match instruction.0 {
            Operation::Acc => accumulator += instruction.1,
            Operation::Jmp => ip += instruction.1 - 1,
            Operation::Nop => (),
        }
        ip += 1;
    }
    accumulator
}

#[aoc(day8, part2)]
fn part2(i: &Vec<(Operation, i64)>) -> i64 {
    let mut input = (*i).clone();
    for i in 0..input.len() {
        //dbg!(i);
        {
            let mut instruction = &mut input[i as usize];
            match instruction.0 {
                Operation::Nop => instruction.0 = Operation::Jmp,
                Operation::Jmp => instruction.0 = Operation::Nop,
                Operation::Acc => continue,
            }
        }
        let mut accumulator = 0;
        let mut ip: i64 = 0;
        let mut visited_instructions = HashSet::new();

        //dbg!(&input);

        while !visited_instructions.contains(&ip) {
            visited_instructions.insert(ip);
            //dbg!(ip);
            let instruction = &input[ip as usize];
            match instruction.0 {
                Operation::Acc => accumulator += instruction.1,
                Operation::Jmp => ip += instruction.1 - 1,
                Operation::Nop => (),
            }
            ip += 1;
            if ip < 0 || ip > input.len() as i64 {
                break;
            }
            if ip as usize == input.len() {
                return accumulator;
            }
        }

        let mut instruction = &mut input[i as usize];
        match instruction.0 {
            Operation::Nop => instruction.0 = Operation::Jmp,
            Operation::Jmp => instruction.0 = Operation::Nop,
            _ => unreachable!(),
        }
    }
    unreachable!();
}
