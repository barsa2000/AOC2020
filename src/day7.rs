use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Result<HashMap<String, HashMap<String, u64>>, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| {
            let mut split = l.split(" contain ");

            let bag = split.next().unwrap().trim_end_matches(" bags");
            let contains = split
                .next()
                .unwrap()
                .split(", ")
                .map(|b| {
                    let split: Vec<&str> = b.splitn(2,' ').collect();

                    if let Ok(qty) = split[0].parse::<u64>() {
                        Some((
                            split[1]
                                .trim_end_matches('.')
                                .trim_end_matches(" bag")
                                .trim_end_matches(" bags")
                                .to_string(),
                            qty,
                        ))
                    } else {
                        None
                    }
                })
                .filter(|b| b.is_some())
                .flatten()
                .collect();

            (bag.to_string(), contains)
        })
        .collect())
}

#[aoc(day7, part1)]
fn part1(bags: &HashMap<String, HashMap<String, u64>>) -> usize {
    bags.iter()
        .filter(|(a, _)| count_bags(bags, a) != 0)
        .count()
}

fn count_bags(bags: &HashMap<String, HashMap<String, u64>>, bag: &str) -> u64 {
    lazy_static! {
        static ref RESULTS: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
    }
    if bag == "shiny gold" {
        return 0;
    }
    let b = bags.get(bag);

    let aa = b.unwrap().get("shiny gold").unwrap_or(&0_u64).to_owned();
    let ab: u64 = b
        .unwrap()
        .iter()
        .map(|(a, b)| {
            if RESULTS.lock().unwrap().get(a).is_some() {
                RESULTS.lock().unwrap().get(a).unwrap().to_owned()
            } else {
                let res = b * count_bags(bags, a);
                RESULTS.lock().unwrap().insert(a.to_owned(), res);
                res
            }
        })
        .sum();
    aa + ab
}

#[aoc(day7, part2)]
fn part2(bags: &HashMap<String, HashMap<String, u64>>) -> u64 {
    count_bags_a(bags, &"shiny gold") - 1
}

fn count_bags_a(bags: &HashMap<String, HashMap<String, u64>>, bag: &str) -> u64 {
    let b = bags.get(bag).unwrap();
    if !b.is_empty() {
        let aa = b
            .iter()
            .map(|(a, b)| b * count_bags_a(bags, a))
            .sum::<u64>();
        aa + 1
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(part1(&parse_input(input).unwrap()), 4);
    }

    #[test]
    fn test_2_1() {
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(part2(&parse_input(input).unwrap()), 32);
    }

    #[test]
    fn test_2_2() {
        let input = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        // println!("{:?}", parse_input(input).unwrap());
        // assert!(false);
        assert_eq!(part2(&parse_input(input).unwrap()), 126);
    }
}
