use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::error::Error;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    input
        .split("\n\n")
        .map(|g| Ok(g.chars().collect()))
        .collect()
}

#[aoc(day6, part1)]
fn part1(passes: &[String]) -> usize {
    passes
        .iter()
        .map(|g| {
            let a: HashSet<char> = g.chars().filter(|c| *c != '\n').collect();
            a.len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(passes: &[String]) -> usize {
    passes
        .iter()
        .map(|g| {
            g.lines()
                .fold(('a'..='z').collect::<HashSet<_>>(), |s, l| {
                    s.intersection(&l.chars().collect())
                        .copied()
                        .collect()
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(part1(&parse_input(input).unwrap()), 11);
    }
    #[test]
    fn test_1_2() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(part2(&parse_input(input).unwrap()), 6);
    }
}
