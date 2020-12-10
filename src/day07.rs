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
            let sub_bags = split
                .next()
                .unwrap()
                .split(", ")
                .map(|b| {
                    let split: Vec<&str> = b.splitn(2, ' ').collect();

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

            (bag.to_string(), sub_bags)
        })
        .collect())
}

#[aoc(day7, part1)]
fn part1(bags: &HashMap<String, HashMap<String, u64>>) -> usize {
    bags.iter()
        .filter(|(bag, _)| count_bags_part1(bags, bag) != 0)
        .count()
}

fn count_bags_part1(bags: &HashMap<String, HashMap<String, u64>>, bag_str: &str) -> u64 {
    lazy_static! {
        static ref RESULTS: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
    }
    if bag_str == "shiny gold" {
        return 0;
    }
    let bag = bags.get(bag_str);

    bag.unwrap().get("shiny gold").unwrap_or(&0_u64).to_owned()
        + bag
            .unwrap()
            .iter()
            .map(|(a, b)| {
                if RESULTS.lock().unwrap().get(a).is_some() {
                    RESULTS.lock().unwrap().get(a).unwrap().to_owned()
                } else {
                    let res = b * count_bags_part1(bags, a);
                    RESULTS.lock().unwrap().insert(a.to_owned(), res);
                    res
                }
            })
            .sum::<u64>()
}

#[aoc(day7, part2)]
fn part2(bags: &HashMap<String, HashMap<String, u64>>) -> u64 {
    count_bags_part2(bags, &"shiny gold") - 1
}

fn count_bags_part2(bags: &HashMap<String, HashMap<String, u64>>, bag_str: &str) -> u64 {
    let sub_bags = bags.get(bag_str).unwrap();
    if !sub_bags.is_empty() {
        sub_bags
            .iter()
            .map(|(sub_bag_str, sub_bag_qty)| sub_bag_qty * count_bags_part2(bags, sub_bag_str))
            .sum::<u64>()
            + 1
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
