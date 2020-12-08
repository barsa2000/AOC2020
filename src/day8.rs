use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Op {
    Nop,
    Jmp,
    Acc,
}

type Instruction = (Op, i32);

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
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
            (op, v)
        })
        .collect::<Vec<Instruction>>())
}

#[aoc(day8, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    let mut acc = 0_i32;
    let mut index = 0_usize;
    let mut done = vec![false; instructions.len()];
    while !done[index] {
        done[index] = true;
        match instructions[index].0 {
            Op::Nop => index += 1,
            Op::Acc => {
                acc += instructions[index].1;
                index += 1;
            }
            Op::Jmp => index = (index as i32 + instructions[index].1) as usize,
        }
    }
    acc
}

#[aoc(day8, part2)]
fn part2(instructions: &[Instruction]) -> i32 {
    let mut acc;
    let mut index;
    let mut changed = 0;

    loop {
        let mut ins = instructions.to_vec();
        let p = changed
            + ins
                .iter()
                .skip(changed)
                .position(|(a, _)| matches!(a, Op::Nop | Op::Jmp))
                .unwrap();

        match ins[p].0 {
            Op::Nop => {
                ins[p].0 = Op::Jmp;
                changed = p + 1;
            }
            Op::Jmp => {
                ins[p].0 = Op::Nop;
                changed = p + 1;
            }
            Op::Acc => unreachable!(),
        }

        acc = 0;
        index = 0;
        let mut done = vec![false; ins.len()];
        while !done[index] {
            done[index] = true;
            match ins[index].0 {
                Op::Nop => index += 1,
                Op::Acc => {
                    acc += ins[index].1;
                    index += 1;
                }
                Op::Jmp => index = (index as i32 + ins[index].1) as usize,
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
