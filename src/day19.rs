use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Op {
    Or(Box<Op>, Box<Op>),
    Concat(Vec<u64>),
    Lit(String),
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (HashMap<u64, Op>, Vec<String>) {
    let mut split = input.splitn(2, "\n\n");

    let rules = split.next().unwrap();
    let strings = split.next().unwrap();

    let rules: HashMap<u64, Op> = rules
        .lines()
        .map(|l| {
            let mut split = l.splitn(2, ": ");
            let k: u64 = split.next().unwrap().parse().unwrap();
            let rule = split.next().unwrap();

            let out;
            if rule.find('|').is_some() {
                let mut split = rule.splitn(2, " | ");
                let left = Box::new(Op::Concat(
                    split
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                ));
                let right = Box::new(Op::Concat(
                    split
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                ));
                out = Op::Or(left, right)
            } else if rule.find('\"').is_some() {
                out = Op::Lit(rule.trim_matches('\"').parse().unwrap());
            } else {
                out = Op::Concat(
                    rule.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                );
            }
            (k, out)
        })
        .collect();

    (rules, strings.lines().map(|l| l.to_string()).collect())
}

#[aoc(day19, part1)]
fn part1(rules_strings: &(HashMap<u64, Op>, Vec<String>)) -> u64 {
    let rules = &rules_strings.0;
    let strings = &rules_strings.1;

    let regex_string = format!("^{}$", regex_string_from_rule(&rules, &0, 1));
    let regex = Regex::new(regex_string.as_str()).unwrap();

    // println!("{:?}", regex);

    strings.iter().filter(|s| regex.is_match(s)).count() as u64
}

#[aoc(day19, part2)]
fn part2(rules_strings: &(HashMap<u64, Op>, Vec<String>)) -> u64 {
    let mut rules = rules_strings.0.clone();
    rules.insert(
        8,
        Op::Or(
            Box::new(Op::Concat(vec![42])),
            Box::new(Op::Concat(vec![42, 8])),
        ),
    );
    rules.insert(
        11,
        Op::Or(
            Box::new(Op::Concat(vec![42, 31])),
            Box::new(Op::Concat(vec![42, 11, 31])),
        ),
    );
    let strings = &rules_strings.1;

    let regex_string = format!("^{}$", regex_string_from_rule(&rules, &0, 10));
    let regex = Regex::new(regex_string.as_str()).unwrap();

    // println!("{:?}", regex);

    strings.iter().filter(|s| regex.is_match(s)).count() as u64
}

fn regex_string_from_rule(rules: &HashMap<u64, Op>, rule_id: &u64, max: u64) -> String {
    match rules.get(&rule_id).unwrap() {
        Op::Or(left, right) => {
            let left_str: String;
            match left.as_ref() {
                Op::Concat(ids) => {
                    left_str = ids
                        .iter()
                        .map(|id| {
                            if max == 0 {
                                "".to_string()
                            } else if id == rule_id {
                                regex_string_from_rule(rules, &id, max - 1)
                            } else {
                                regex_string_from_rule(rules, &id, 10)
                            }
                        })
                        .collect()
                }
                _ => unimplemented!(),
            };

            let right_str: String;
            match right.as_ref() {
                Op::Concat(ids) => {
                    right_str = ids
                        .iter()
                        .map(|id| {
                            if max == 0 {
                                "".to_string()
                            } else if id == rule_id {
                                regex_string_from_rule(rules, &id, max - 1)
                            } else {
                                regex_string_from_rule(rules, &id, 10)
                            }
                        })
                        .collect()
                }
                _ => unimplemented!(),
            };

            format!("({}|{})", left_str, right_str)
        }
        Op::Concat(ids) => ids
            .iter()
            .map(|id| {
                if max == 0 {
                    "".to_string()
                } else if id == rule_id {
                    regex_string_from_rule(rules, &id, max - 1)
                } else {
                    regex_string_from_rule(rules, &id, 10)
                }
            })
            .collect::<String>(),
        Op::Lit(s) => s.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(part1(&parse_input(input)), 2);
    }

    #[test]
    fn test_2_1() {
        let input = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(part2(&parse_input(input)), 12);
    }
}
