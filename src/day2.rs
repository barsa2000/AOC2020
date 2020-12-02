use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

type PwdPolicy = (usize, usize, char);

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Result<Vec<(PwdPolicy, String)>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(": ");
            let pol = split.next().unwrap();
            let s = split.next().unwrap().to_string();

            let mut split = pol.split(" ");
            let range = split.next().unwrap();
            let c = split.next().unwrap().chars().next().unwrap();

            let mut split = range.split("-");
            let min = split.next().unwrap().parse::<usize>().unwrap();
            let max = split.next().unwrap().parse::<usize>().unwrap();

            Ok(((min, max, c), s))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(passwords: &Vec<(PwdPolicy, String)>) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let count = p.1.chars().filter(|c| *c == p.0 .2).count();
            count >= p.0 .0 && count <= p.0 .1
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(passwords: &Vec<(PwdPolicy, String)>) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let s = p.1[p.0 .0 - 1..p.0 .1].to_string();
            s.starts_with(p.0 .2) ^ s.ends_with(p.0 .2)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input("1-3 a: abcde").unwrap()), 1);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input("1-3 a: abcde").unwrap()), 1);
    }

    #[test]
    fn sample3() {
        assert_eq!(part2(&parse_input("1-3 b: cdefg").unwrap()), 0);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2(&parse_input("2-9 c: ccccccccc").unwrap()), 0);
    }
}
