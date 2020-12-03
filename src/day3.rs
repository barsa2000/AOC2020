use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    //true if tree
    let mut out = vec![];

    input.lines().for_each(|l| {
        let mut row = vec![];
        l.chars().for_each(|c| match c {
            '.' => row.push(false),
            '#' => row.push(true),
            _ => (),
        });
        out.push(row);
    });

    Ok(out)
}

fn get_trees(map: &Vec<Vec<bool>>, speed_x: usize, speed_y: usize) -> u64 {
    let mut c = 0_u64;

    let mut x = 0;
    let mut y = 0;

    let w = map[0].len();

    loop {
        if y >= map.len() {
            break;
        }

        if map[y][x % w] {
            c += 1;
        }

        x += speed_x;
        y += speed_y
    }

    c
}

#[aoc(day3, part1)]
fn part1(map: &Vec<Vec<bool>>) -> u64 {
    get_trees(map, 3, 1)
}

#[aoc(day3, part2)]
fn part2(map: &Vec<Vec<bool>>) -> u64 {
    get_trees(map, 1, 1)
        * get_trees(map, 3, 1)
        * get_trees(map, 5, 1)
        * get_trees(map, 7, 1)
        * get_trees(map, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        assert_eq!(part1(&parse_input(input).unwrap()), 7);
    }

    #[test]
    fn sample2() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        assert_eq!(part2(&parse_input(input).unwrap()), 336);
    }
}
