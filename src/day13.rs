use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (i128, Vec<(usize, i128)>) {
    let t = input.lines().next().unwrap().parse().unwrap();
    let ids = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, v)| *v != "x")
        .map(|(i, v)| (i, v.parse::<i128>().unwrap()))
        .collect();

    (t, ids)
}

#[aoc(day13, part1)]
fn part1(input: &(i128, Vec<(usize, i128)>)) -> i128 {
    let t = input.0;
    let ids = &input.1;
    let min = ids
        .iter()
        .enumerate()
        .map(|(i, (_, id))| (i, id - (t % id)))
        .min_by(|&(_, a), &(_, b)| a.cmp(&b))
        .unwrap();

    ids[min.0].1 * min.1
}

#[aoc(day13, part2)]
fn part2(input: &(i128, Vec<(usize, i128)>)) -> i128 {
    let modulii: Vec<i128> = input.1.iter().map(|a| a.1).collect();
    let residues: Vec<i128> = input.1.iter().map(|a| a.0 as i128).collect();

    modulii.iter().product::<i128>() - chinese_remainder(&residues, &modulii).unwrap()
}

#[allow(clippy::clippy::many_single_char_names)]
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
939
7,13,x,x,59,x,31,19";
        assert_eq!(part1(&parse_input(input)), 295);
    }

    #[test]
    fn test_2_1() {
        let input = "\
939
7,13,x,x,59,x,31,19";
        assert_eq!(part2(&parse_input(input)), 1068781);
    }

    #[test]
    fn test_2_2() {
        let input = "\
939
17,x,13,19";
        assert_eq!(part2(&parse_input(input)), 3417);
    }
}
