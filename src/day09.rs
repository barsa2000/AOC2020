use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::error::Error;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    Ok(input.lines().map(|l| l.parse().unwrap()).collect())
}

#[aoc(day9, part1)]
fn part1(nums: &[u64]) -> u64 {
    solve1(nums, 25)
}

fn solve1(nums: &[u64], window_size: usize) -> u64 {
    let s = window_size;
    nums.windows(s + 1)
        .find(|n| {
            !n.iter().take(s).any(|i| {
                n.iter()
                    .take(s)
                    .any(|j| *i != *j && *n.iter().last().unwrap() == *i + *j)
            })
        })
        .unwrap()
        .last()
        .copied()
        .unwrap()
}

#[aoc(day9, part1, faster)]
fn part1_faster(nums: &[u64]) -> u64 {
    solve1_faster(nums, 25)
}

fn solve1_faster(nums: &[u64], window_size: usize) -> u64 {
    let s = window_size;
    let mut i = 0;

    while i + s < nums.len() {
        let mut found = false;
        'outer: for j in 0..s - 1 {
            for k in j + 1..s {
                if nums[i + j] != nums[i + k] && nums[i + j] + nums[i + k] == nums[i + s] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return nums[i + s];
        }
        i += 1;
    }

    panic!("nothing found")
}

#[aoc(day9, part2)]
fn part2(nums: &[u64]) -> u64 {
    solve2(nums, 25)
}

fn solve2(nums: &[u64], window_size: usize) -> u64 {
    let sum = solve1(nums, window_size);

    for i in 0..nums.len() - 1 {
        let mut j = i + 1;
        while nums[i..=j].iter().sum::<u64>() < sum {
            j += 1;
        }
        if nums[i..=j].iter().sum::<u64>() == sum {
            return nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap();
        }
    }

    panic!("nothing found")
}

#[aoc(day9, part2, faster)]
fn part2_faster(nums: &[u64]) -> u64 {
    solve2_faster(nums, 25)
}

fn solve2_faster(nums: &[u64], window_size: usize) -> u64 {
    let sum = solve1_faster(nums, window_size);
    let mut i = 0;
    let mut j = 0;
    let mut s = nums[i];

    while i < nums.len() && j < nums.len() {
        match s.cmp(&sum) {
            Ordering::Greater => {
                s -= nums[i];
                i += 1;
            }
            Ordering::Less => {
                j += 1;
                s += nums[j];
            }
            Ordering::Equal => {
                return nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap()
            }
        }
    }

    panic!("nothing found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(solve1(&parse_input(input).unwrap(), 5), 127);
    }

    #[test]
    fn test_1_2() {
        let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(solve1_faster(&parse_input(input).unwrap(), 5), 127);
    }

    #[test]
    fn test_2_1() {
        let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(solve2(&parse_input(input).unwrap(), 5), 62);
    }

    #[test]
    fn test_2_2() {
        let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        // assert_eq!(part2_faster(&parse_input(input).unwrap()), 40);
        assert_eq!(solve2_faster(&parse_input(input).unwrap(), 5), 62);
    }
}
