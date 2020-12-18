use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Vec<(bool, String)>> {
    input
        .lines()
        .map(|l| {
            let mut num = String::new();
            let mut out: Vec<(bool, String)> = vec![];
            l.chars().for_each(|c| match c {
                '+' | '*' | '(' | ')' => {
                    if !num.is_empty() {
                        out.push((true, num.clone()));
                        num = String::new();
                    }
                    out.push((false, c.to_string()));
                }
                _ if c.is_numeric() => num.push(c),
                _ => (),
            });
            if num.parse::<u64>().is_ok() {
                out.push((true, num));
            }
            out
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(expressions: &[Vec<(bool, String)>]) -> u64 {
    let mut postfix: Vec<Vec<(bool, String)>> = expressions
        .iter()
        .map(|tokens| {
            let mut out = vec![];
            let mut op_stack = vec![];

            tokens.iter().for_each(|t| {
                if t.0 {
                    out.push(t.clone());
                } else {
                    match t.1.as_str() {
                        "(" => {
                            op_stack.push(t.clone());
                        }
                        ")" => {
                            let mut top = op_stack.pop().unwrap();
                            while top.1 != "(" {
                                out.push(top.clone());
                                top = op_stack.pop().unwrap();
                            }
                        }
                        _ => {
                            while !op_stack.is_empty() && op_stack.last().unwrap().1 != "(" {
                                // && op_stack.last().unwrap().1 == "+"
                                out.push(op_stack.pop().unwrap());
                            }
                            op_stack.push(t.clone());
                        }
                    }
                }
            });

            while let Some(op) = op_stack.pop() {
                out.push(op.clone());
            }

            out
        })
        .collect();

    postfix.iter_mut().map(|tokens| solve_postfix(tokens)).sum()
}

#[aoc(day18, part2)]
fn part2(expressions: &[Vec<(bool, String)>]) -> u64 {
    let mut postfix: Vec<Vec<(bool, String)>> = expressions
        .iter()
        .map(|tokens| {
            let mut out = vec![];
            let mut op_stack = vec![];

            tokens.iter().for_each(|t| {
                if t.0 {
                    out.push(t.clone());
                } else {
                    match t.1.as_str() {
                        "(" => {
                            op_stack.push(t.clone());
                        }
                        ")" => {
                            let mut top = op_stack.pop().unwrap();
                            while top.1 != "(" {
                                out.push(top.clone());
                                top = op_stack.pop().unwrap();
                            }
                        }
                        _ => {
                            while !op_stack.is_empty() && op_stack.last().unwrap().1 == "+" {
                                out.push(op_stack.pop().unwrap());
                            }
                            op_stack.push(t.clone());
                        }
                    }
                }
            });

            while let Some(op) = op_stack.pop() {
                out.push(op.clone());
            }

            out
        })
        .collect();

    postfix.iter_mut().map(|tokens| solve_postfix(tokens)).sum()
}

fn solve_postfix(tokens: &mut Vec<(bool, String)>) -> u64 {
    while tokens.len() > 1 {
        for i in 0..tokens.len() {
            let tok = tokens.get(i).unwrap().clone();
            if !tok.0 {
                let n1: u64 = tokens.get(i - 2).unwrap().1.parse().unwrap();
                let n2: u64 = tokens.get(i - 1).unwrap().1.parse().unwrap();

                match tok.1.as_str() {
                    "+" => {
                        let n: u64 = n1 + n2;
                        tokens.insert(i + 1, (true, n.to_string()));
                    }
                    "*" => {
                        let n: u64 = n1 * n2;
                        tokens.insert(i + 1, (true, n.to_string()));
                    }
                    _ => panic!(),
                }
                tokens.remove(i);
                tokens.remove(i - 1);
                tokens.remove(i - 2);
                break;
            }
        }
    }
    tokens.first().unwrap().1.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(part1(&parse_input(input)), 71);
    }

    #[test]
    fn test_1_2() {
        let input = "2 * 3 + (4 * 5)";
        assert_eq!(part1(&parse_input(input)), 26);
    }
}
