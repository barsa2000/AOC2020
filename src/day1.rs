use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    input.lines().map(|l|{
        Ok(l.parse::<u32>().unwrap())
    }).collect()
}

#[aoc(day1, part1)]
fn part1(expenses: &[u32]) -> u32 {

    for i in 0..expenses.len()-1 {
        for j in i..expenses.len()-1{
            if expenses[i] + expenses[j] == 2020{
                return expenses[i] * expenses[j];
            }
        }
    }
    return 0;
}

#[aoc(day1, part2)]
fn part2(expenses: &[u32]) -> u32 {
    for i in 0..expenses.len()-1 {
        for j in i..expenses.len()-1{
            for k in j ..expenses.len()-1 {
                if expenses[i] + expenses[j]+ expenses[k] == 2020{
                    return expenses[i] * expenses[j] * expenses[k];
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn sample2() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&input), 241861950);
    }

}
