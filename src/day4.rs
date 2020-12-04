use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::error::Error;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    Ok(input
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|entry| {
                    let mut s = entry.split(":");
                    (s.next().unwrap().to_string() , s.next().unwrap().to_string())
                })
                .collect()
        })
        .collect())
}

#[aoc(day4, part1)]
fn part1(passports: &Vec<HashMap<String, String>>) -> usize {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; //,"cid"

    passports
        .iter()
        .filter(|p| keys.iter().all(|k| p.contains_key(*k)))
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &Vec<HashMap<String, String>>) -> usize {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; //,"cid"

    let byr_valids = 1920..=2002;
    let iyr_valids = 2010..=2020;
    let eyr_valids = 2020..=2030;
    let ecl_valids = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let hgt_cm_valids = 150..=193;
    let hgt_in_valids = 59..=76;

    passports.iter().filter(|p|{
        keys.iter().all(|k| p.contains_key(*k)) &&
        (byr_valids.contains(&p.get("byr").unwrap().parse::<u128>().unwrap())) && //byr
        (iyr_valids.contains(&p.get("iyr").unwrap().parse::<u128>().unwrap())) && //iyr
        (eyr_valids.contains(&p.get("eyr").unwrap().parse::<u128>().unwrap())) && //eyr
        (ecl_valids.contains(&&p.get("ecl").unwrap()[..])) && //ecl
        (p.get("pid").unwrap().len() == 9 && p.get("pid").unwrap().chars().all(|c| c.is_digit(10))) && //pid
        (p.get("hcl").unwrap().len() == 7 && p.get("hcl").unwrap().starts_with("#") && p.get("hcl").unwrap().chars().skip(1).all(|c| c.is_digit(16))) && //hcl
        ((p.get("hgt").unwrap().ends_with("cm") && hgt_cm_valids.contains(&p.get("hgt").unwrap().trim_end_matches("cm").parse::<u128>().unwrap())) ||
         (p.get("hgt").unwrap().ends_with("in") && hgt_in_valids.contains(&p.get("hgt").unwrap().trim_end_matches("in").parse::<u128>().unwrap()))) //hgt
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f";

        // println!("{:?}", part2(input));
        // assert_eq!(part2(input), 1);
        assert_eq!(part1(&parse_input(input).unwrap()), 1);
    }
}
