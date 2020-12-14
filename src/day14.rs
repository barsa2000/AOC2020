use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Ops {
    Mask(String),
    Mem(usize, usize),
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Ops> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<op>\w+)(\[(?P<addr>\d+)\])? = (?P<value>[\dX]+)$").unwrap();
    }

    input
        .lines()
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            match captures.name("op").unwrap().as_str() {
                "mask" => Ops::Mask(captures.name("value").unwrap().as_str().to_string()),
                "mem" => Ops::Mem(
                    captures.name("addr").unwrap().as_str().parse().unwrap(),
                    captures.name("value").unwrap().as_str().parse().unwrap(),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(ops: &[Ops]) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut current_mask = "";
    for op in ops {
        match op {
            Ops::Mask(m) => {
                current_mask = m;
            }
            Ops::Mem(addr, val) => {
                mem.insert(*addr, apply_mask_p1(val, current_mask));
            }
        }
    }

    mem.values().sum()
}

fn apply_mask_p1(value: &usize, mask: &str) -> usize {
    let and_mask = mask.replace("X", "1");
    let or_mask = mask.replace("X", "0");

    value & usize::from_str_radix(and_mask.as_str(), 2).unwrap()
        | usize::from_str_radix(or_mask.as_str(), 2).unwrap()
}

#[aoc(day14, part2)]
fn part2(ops: &[Ops]) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut current_mask = "";
    for op in ops {
        match op {
            Ops::Mask(m) => {
                current_mask = m;
            }
            Ops::Mem(addr, val) => {
                for a in apply_mask_p2(addr, current_mask) {
                    mem.insert(a, *val);
                }
            }
        }
    }

    mem.values().sum()
}

fn apply_mask_p2(addr: &usize, mask: &str) -> Vec<usize> {
    let floatings: Vec<usize> = mask
        .char_indices()
        .filter(|&(_, c)| c == 'X')
        .map(|(i, _)| mask.len() - i - 1)
        .rev()
        .collect();

    let mut and_mask: usize = !0;
    for &f in &floatings {
        and_mask &= !(1 << f);
    }

    let masked_addr = *addr & and_mask;

    let base_mask = mask.replace("X", "0");
    let base_mask = usize::from_str_radix(base_mask.as_str(), 2).unwrap();

    let max = 2_usize.pow(floatings.len() as u32);
    let mut out: Vec<usize> = vec![];

    for n in 0..max {
        let mut mask = base_mask;
        for (i, &f) in floatings.iter().enumerate() {
            mask |= ((n >> i) & 1) << f;
        }
        out.push(masked_addr | mask);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(part1(&parse_input(input)), 165);
    }

    #[test]
    fn test_2_1() {
        let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part2(&parse_input(input)), 208);
    }
}
