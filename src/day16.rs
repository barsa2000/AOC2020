use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[allow(non_camel_case_types)]
type Rules_MyTicket_NearbyTickets = (HashMap<String, Vec<(u64, u64)>>, Vec<u64>, Vec<Vec<u64>>);

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Rules_MyTicket_NearbyTickets {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let mut my_ticket = split.next().unwrap().lines().skip(1);
    let nearby_tickets = split.next().unwrap().lines().skip(1);

    let rules = rules
        .lines()
        .map(|l| {
            let mut split = l.splitn(2, ": ");
            let field = split.next().unwrap().to_string();
            let ranges: Vec<(u64, u64)> = split
                .next()
                .unwrap()
                .split(" or ")
                .map(|r| {
                    let mut split = r.splitn(2, '-');
                    let min = split.next().unwrap().parse::<u64>().unwrap();
                    let max = split.next().unwrap().parse::<u64>().unwrap();
                    (min, max)
                })
                .collect();
            (field, ranges)
        })
        .collect();

    let my_ticket = my_ticket
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let nearby_tickets = nearby_tickets
        .map(|l| l.split(',').map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();

    (rules, my_ticket, nearby_tickets)
}

#[aoc(day16, part1, real)]
fn part1(input: &Rules_MyTicket_NearbyTickets) -> u64 {
    let rules = input.0.clone();
    let nearby = input.2.clone();

    nearby
        .iter()
        .map(|col| {
            col.iter()
                .filter(|&&n| {
                    rules
                        .values()
                        .flatten()
                        .all(|(min, max)| n < *min || n > *max)
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[aoc(day16, part2, real)]
fn part2(input: &Rules_MyTicket_NearbyTickets) -> u64 {
    let mut rules = input.0.clone();
    let my_ticket = input.1.clone();
    let nearby = input.2.clone();

    let filtered: Vec<Vec<u64>> = nearby
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|&n| {
                rules
                    .values()
                    .flatten()
                    .any(|(min, max)| n >= *min && n <= *max)
            })
        })
        .cloned()
        .collect();

    let mut nearby_tickets_cols: Vec<(usize, Vec<u64>)> = vec![];

    for i in 0..filtered.first().unwrap().len() {
        nearby_tickets_cols.push((i, vec![]));
    }

    filtered.iter().for_each(|l| {
        let mut i = 0;
        l.iter().for_each(|&n| {
            nearby_tickets_cols[i].1.push(n);
            i += 1;
        });
    });

    let mut rules_indexes: HashMap<String, usize> = HashMap::new();
    let ticket_count = filtered.len();

    while !nearby_tickets_cols.is_empty() {
        let mut counts: HashMap<String, Vec<usize>> = rules
            .iter()
            .map(|(k, _)| (k.clone(), vec![0; nearby_tickets_cols.len()]))
            .collect();

        rules.iter().for_each(|(key, ranges)| {
            nearby_tickets_cols
                .iter()
                .enumerate()
                .for_each(|(i, (_, col))| {
                    counts.get_mut(key.as_str()).unwrap()[i] = col
                        .iter()
                        .filter(|&&n| ranges.iter().any(|&(min, max)| n >= min && n <= max))
                        .count();
                });
        });

        let min_of_max_key: String = counts
            .iter()
            .map(|(k, c)| (k.clone(), c.iter().filter(|&&n| n == ticket_count).count()))
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0;

        let min_of_max_col = counts
            .get(&min_of_max_key.clone())
            .unwrap()
            .iter()
            .position(|&a| a == ticket_count)
            .unwrap();

        rules_indexes.insert(min_of_max_key.clone(), min_of_max_col);

        rules.remove(&min_of_max_key.clone());
        nearby_tickets_cols.remove(min_of_max_col);
    }

    rules_indexes
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| my_ticket[*v])
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part1(&parse_input(input)), 71);
    }

    #[test]
    fn test_2_1() {
        let input = "\
departure a: 0-1 or 4-19
departure b: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        assert_eq!(part2(&parse_input(input)), 11 * 12);
    }
}
