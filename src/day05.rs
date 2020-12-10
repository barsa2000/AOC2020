use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

#[aoc_generator(day5)]
fn parse_input_bin(input: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| {
            Ok(l.chars().fold(0_u16, |n, c| {
                n << 1
                    | match c {
                        'R' | 'B' => 1,
                        _ => 0,
                    }
            }))
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1_bin(passes: &[u16]) -> u16 {
    *passes.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn part2_bin(passes: &[u16]) -> u16 {
    // *passes.iter().max().unwrap()
    let mut ids = passes.to_owned();
    ids.sort_unstable();
    ids.windows(2).find(|x| x[1] != x[0] + 1).unwrap()[0] + 1
}

#[aoc_generator(day5, part2, orig)]
#[aoc_generator(day5, part1, orig)]
fn parse_input_orig(input: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| Ok(calc_row(l) * 8 + calc_col(l)))
        .collect()
}

#[aoc(day5, part1, orig)]
fn part1_orig(passes: &[u16]) -> u16 {
    *passes.iter().max().unwrap()
}

#[aoc(day5, part2, orig)]
fn part2_orig(passes: &[u16]) -> u16 {
    let mut ids = passes.to_owned();
    ids.sort_unstable();
    ids.iter()
        .zip(ids.iter().skip(1))
        .find(|(i, n)| **n != **i + 1)
        .unwrap_or((&0, &0))
        .0
        + 1
}

fn calc_row(s: &str) -> u16 {
    binary_space_part(
        s.chars().take(7).collect::<String>().as_str(),
        0,
        128,
        'B',
        'F',
    )
}

fn calc_col(s: &str) -> u16 {
    binary_space_part(
        s.chars().skip(7).collect::<String>().as_str(),
        0,
        8,
        'R',
        'L',
    )
}

fn binary_space_part(code: &str, start: u16, len: u16, upper_c: char, lower_c: char) -> u16 {
    if code.is_empty() {
        return start;
    }

    let p;

    let c = code.chars().next().unwrap();
    if c == lower_c {
        p = start
    } else if c == upper_c {
        p = start + len / 2
    } else {
        unreachable!();
    }

    binary_space_part(
        code.chars().skip(1).collect::<String>().as_str(),
        p,
        len / 2,
        upper_c,
        lower_c,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "BFFFBBFRRR";

        assert_eq!(part1_bin(&parse_input_bin(input).unwrap()), 567);
    }

    #[test]
    fn test_1_1_orig() {
        let input = "BFFFBBFRRR";

        assert_eq!(part1_bin(&parse_input_bin(input).unwrap()), 567);
    }

    #[test]
    fn test_2_1() {
        let input = "FFFBBBFRRR";

        assert_eq!(part1_bin(&parse_input_bin(input).unwrap()), 119);
    }

    #[test]
    fn test_2_1_orig() {
        let input = "FFFBBBFRRR";

        assert_eq!(part1_bin(&parse_input_bin(input).unwrap()), 119);
    }

    #[test]
    fn test_3_1() {
        let input = "BBFFBBFRLL";

        assert_eq!(part1_bin(&parse_input_bin(input).unwrap()), 820);
    }

    #[test]
    fn test_3_1_orig() {
        let input = "BBFFBBFRLL";

        assert_eq!(part1_bin(&parse_input_bin(input).unwrap()), 820);
    }
}
