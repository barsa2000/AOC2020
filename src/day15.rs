use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<u64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
fn part1(starting_numbers: &[u64]) -> u64 {
    solve(starting_numbers, 2020)
}

#[aoc(day15, part2)]
fn part2(starting_numbers: &[u64]) -> u64 {
    solve(starting_numbers, 30000000)
}

fn solve(starting_numbers: &[u64], max: u64) -> u64 {
    let mut numbers: HashMap<u64, u64> = HashMap::new();

    starting_numbers
        .iter()
        .take(starting_numbers.len() - 1)
        .enumerate()
        .for_each(|(i, n)| {
            numbers.insert(*n, i as u64);
        });

    let mut turn = starting_numbers.len() as u64;

    let mut last_spoken: u64 = *starting_numbers.iter().last().unwrap();

    while turn < max {
        let curr;

        if let Some(a) = numbers.get(&last_spoken) {
            curr = turn - a - 1;
        } else {
            curr = 0;
        }
        numbers.insert(last_spoken, turn - 1);
        last_spoken = curr;
        turn += 1;
    }

    last_spoken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
0,3,6";
        assert_eq!(part1(&parse_input(input)), 436);
    }

    #[test]
    fn test_2_1() {
        let input = "\
0,3,6";
        assert_eq!(part2(&parse_input(input)), 175594);
    }
}
