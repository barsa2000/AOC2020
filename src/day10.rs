use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::error::Error;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut output = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>();
    output.sort_unstable();
    output.insert(0, 0);
    output.push(output.last().unwrap() + 3);
    Ok(output)
}

#[aoc(day10, part1)]
fn part1(adapters: &[i32]) -> i32 {
    let mut diffs = vec![0; 3];

    adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .for_each(|(a, b)| diffs[(*b - *a - 1) as usize] += 1);

    diffs[0] * diffs[2]
}

#[aoc(day10, part2)]
fn part2(adapters: &[i32]) -> u64 {
    let mut results = HashMap::new();
    count_paths(&adapters, &mut results, 0)
}

fn count_paths(adapters: &[i32], results: &mut HashMap<usize, u64>, index: usize) -> u64 {
    if index >= adapters.len() - 1 {
        1
    } else if let Some(res) = results.get(&index) {
        *res
    } else {
        let paths: u64 = adapters
            .iter()
            .skip(index + 1)
            .take(3)
            .enumerate()
            .filter(|(_, a)| **a - adapters[index] <= 3)
            .map(|(i, _)| count_paths(adapters, results, index + i + 1))
            .sum::<u64>();

        results.insert(index, paths);

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(part1(&parse_input(input).unwrap()), 35);
    }

    #[test]
    fn test_1_2() {
        let input = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part1(&parse_input(input).unwrap()), 220);
    }

    #[test]
    fn test_2_1() {
        let input = "\
16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(part2(&parse_input(input).unwrap()), 8);
    }

    #[test]
    fn test_2_2() {
        let input = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part2(&parse_input(input).unwrap()), 19208);
    }
}
