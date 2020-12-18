use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day17)]
fn parse_input(input: &str) -> HashSet<(i64, i64, i64, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as i64, y as i64, 0, 0))
                .collect::<Vec<(i64, i64, i64, i64)>>()
        })
        .flatten()
        .collect()
}

#[aoc(day17, part1)]
fn part1(cubes: &HashSet<(i64, i64, i64, i64)>) -> u64 {
    let mut cubes = cubes.clone();
    for _ in 0..6 {
        let mut tmp = cubes.clone();

        for cube in &cubes {
            let neighbors = count_active_neighbors_3d(&cubes, cube);
            if neighbors != 2 && neighbors != 3 {
                tmp.remove(cube);
            }
            check_inactive_neighbors_3d(&cubes, cube)
                .iter()
                .for_each(|c| {
                    tmp.insert(*c);
                });
        }

        cubes = tmp.clone();
    }

    cubes.len() as u64
}

#[aoc(day17, part2)]
fn part2(cubes: &HashSet<(i64, i64, i64, i64)>) -> u64 {
    let mut cubes = cubes.clone();
    for _ in 0..6 {
        let mut tmp = cubes.clone();

        for cube in &cubes {
            let neighbors = count_active_neighbors_4d(&cubes, cube);
            if neighbors != 2 && neighbors != 3 {
                tmp.remove(cube);
            }
            check_inactive_neighbors_4d(&cubes, cube)
                .iter()
                .for_each(|c| {
                    tmp.insert(*c);
                });
        }

        cubes = tmp.clone();
    }

    cubes.len() as u64
}

fn check_inactive_neighbors_3d(
    cubes: &HashSet<(i64, i64, i64, i64)>,
    pos: &(i64, i64, i64, i64),
) -> Vec<(i64, i64, i64, i64)> {
    let mut out = vec![];
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                let p = (pos.0 + x, pos.1 + y, pos.2 + z, 0);
                if p != *pos && cubes.get(&p).is_none() {
                    let neighbors = count_active_neighbors_3d(&cubes, &p);
                    if neighbors == 3 {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

fn count_active_neighbors_3d(
    cubes: &HashSet<(i64, i64, i64, i64)>,
    pos: &(i64, i64, i64, i64),
) -> u32 {
    let mut tot = 0;
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                let p = (pos.0 + x, pos.1 + y, pos.2 + z, 0);
                if p != *pos && cubes.get(&p).is_some() {
                    tot += 1;
                }
            }
        }
    }
    tot
}

fn check_inactive_neighbors_4d(
    cubes: &HashSet<(i64, i64, i64, i64)>,
    pos: &(i64, i64, i64, i64),
) -> Vec<(i64, i64, i64, i64)> {
    let mut out = vec![];
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let p = (pos.0 + x, pos.1 + y, pos.2 + z, pos.3 + w);
                    if p != *pos && cubes.get(&p).is_none() {
                        let neighbors = count_active_neighbors_4d(&cubes, &p);
                        if neighbors == 3 {
                            out.push(p);
                        }
                    }
                }
            }
        }
    }
    out
}

fn count_active_neighbors_4d(
    cubes: &HashSet<(i64, i64, i64, i64)>,
    pos: &(i64, i64, i64, i64),
) -> u32 {
    let mut tot = 0;
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let p = (pos.0 + x, pos.1 + y, pos.2 + z, pos.3 + w);
                    if p != *pos && cubes.get(&p).is_some() {
                        tot += 1;
                    }
                }
            }
        }
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
.#.
..#
###";
        assert_eq!(part1(&parse_input(input)), 112);
    }
    #[test]
    fn test_2_1() {
        let input = "\
.#.
..#
###";
        assert_eq!(part2(&parse_input(input)), 848);
    }
}
