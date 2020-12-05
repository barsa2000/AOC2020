use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::error::Error;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    input.lines().map(|l| Ok(l.parse().unwrap())).collect()
}

#[aoc(day1, part1)]
fn part1(expenses: &[i32]) -> i32 {
    for i in 0..expenses.len() - 1 {
        for j in i..expenses.len() - 1 {
            if expenses[i] + expenses[j] == 2020 {
                return expenses[i] * expenses[j];
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
fn part2(expenses: &[i32]) -> i32 {
    for i in 0..expenses.len() - 1 {
        for j in i..expenses.len() - 1 {
            for k in j..expenses.len() - 1 {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return expenses[i] * expenses[j] * expenses[k];
                }
            }
        }
    }
    unreachable!()
}

#[aoc_generator(day1, part1, hash)]
#[aoc_generator(day1, part2, hash)]
fn parse_input_hash(input: &str) -> Result<HashSet<i32>, Box<dyn Error>> {
    input.lines().map(|l| Ok(l.parse().unwrap())).collect()
}

#[aoc(day1, part1, hash)]
fn part1_hash(expenses: &HashSet<i32>) -> i32 {
    for i in expenses {
        let j = 2020 - i;
        if expenses.contains(&j) {
            return j * i;
        }
    }
    unreachable!()
}

#[aoc(day1, part2, hash)]
fn part2_hash(expenses: &HashSet<i32>) -> i32 {
    for i in expenses {
        for j in expenses {
            let k = 2020 - j - i;
            if expenses.contains(&k) {
                return k * j * i;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn test_1_1_hash() {
        let input = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();
        assert_eq!(part1_hash(&input), 514579);
    }

    #[test]
    fn test_1_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&input), 241861950);
    }

    #[test]
    fn test_1_2_hash() {
        let input = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();
        assert_eq!(part2_hash(&input), 241861950);
    }
}
