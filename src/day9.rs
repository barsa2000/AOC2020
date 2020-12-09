use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;
use std::cmp::Ordering;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    Ok(input.lines().map(|l| l.parse().unwrap()).collect())
}

#[aoc(day9, part1)]
fn part1(nums: &[u64]) -> u64 {
    let s = 25;
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

#[aoc(day9, part2)]
fn part2(nums: &[u64]) -> u64 {
    let sum = part1(nums);

    for i in 0..nums.len() - 1 {
        let mut j = i + 1;
        let mut s = 0;
        while s < sum {
            j += 1;
            s += nums[j];
        }

        if s == sum {
            return nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap();
        }
    }

    panic!("nothing found")
}

#[aoc(day9, part2, faster)]
fn part2_faster(nums: &[u64]) -> u64 {
    let sum = part1(nums);
    let mut i = 0;
    let mut j = 0;
    let mut s = nums[i];
    let mut min = nums[i];
    let mut max = nums[i];

    while i < nums.len() && j < nums.len() {

        match s.cmp(&sum) {
            Ordering::Greater => {
                s-= nums[i];
                i+=1;
                if nums[i] < min {
                    min = nums[i];
                }
                if nums[i] > max {
                    max = nums[i];
                }
            },
            Ordering::Less => {
                j+=1;
                s += nums[j];
                if nums[j] < min {
                    min = nums[i];
                }
                if nums[j] > max {
                    max = nums[i];
                }
            },
            Ordering::Equal => return min + max
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
        assert_eq!(part1(&parse_input(input).unwrap()), 127);
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
        assert_eq!(part2(&parse_input(input).unwrap()), 62);
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
        assert_eq!(part2_faster(&parse_input(input).unwrap()), 40);
        // assert_eq!(part2_faster(&parse_input(input).unwrap()), 127);
    }
}
