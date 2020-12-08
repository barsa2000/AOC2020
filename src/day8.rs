use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Op {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Result<Vec<Op>, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let op = match split.next().unwrap() {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                _ => unreachable!(),
            };
            let v = split.next().unwrap().parse::<i32>().unwrap();
            op(v)
        })
        .collect::<Vec<Op>>())
}

#[aoc(day8, part1)]
fn part1(instructions: &[Op]) -> i32 {
    let mut acc = 0_i32;
    let mut index = 0_usize;
    let mut done = vec![false; instructions.len()];
    while !done[index] {
        done[index] = true;
        match instructions[index] {
            Op::Nop(_) => index += 1,
            Op::Acc(n) => {
                acc += n;
                index += 1;
            }
            Op::Jmp(n) => index = (index as i32 + n) as usize,
        }
    }
    acc
}

#[aoc(day8, part2)]
fn part2(instructions: &[Op]) -> i32 {
    let mut acc;
    let mut index;
    let mut changed = 0;

    loop {
        let mut ins = instructions.to_vec();
        let p = changed
            + ins
                .iter()
                .skip(changed)
                .position(|a| matches!(a, Op::Nop(_) | Op::Jmp(_)))
                .unwrap();

        match ins[p] {
            Op::Nop(n) => {
                ins[p] = Op::Jmp(n);
                changed = p + 1;
            }
            Op::Jmp(n) => {
                ins[p] = Op::Nop(n);
                changed = p + 1;
            }
            Op::Acc(_) => unreachable!(),
        }

        acc = 0;
        index = 0;
        let mut done = vec![false; ins.len()];
        while !done[index] {
            done[index] = true;
            match ins[index] {
                Op::Nop(_) => index += 1,
                Op::Acc(n) => {
                    acc += n;
                    index += 1;
                }
                Op::Jmp(n) => index = (index as i32 + n) as usize,
            }
            if index >= done.len() {
                break;
            }
        }
        if index == ins.len() {
            break;
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(part1(&parse_input(input).unwrap()), 5);
    }

    #[test]
    fn test_2_1() {
        let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        // println!("{:?}", part2(&parse_input(input).unwrap()));
        // assert!(false);
        assert_eq!(part2(&parse_input(input).unwrap()), 8);
    }
}
