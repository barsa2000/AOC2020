use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Move {
    North(f64),
    South(f64),
    East(f64),
    West(f64),
    Rotate(f64, i32), //1:clockwise, -1:anticlockwise
    Forward(f64),
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let dir = &l[0..=0];
            let par = l[1..].parse::<f64>().unwrap();
            match dir {
                "N" => Move::North(par),
                "S" => Move::South(par),
                "E" => Move::East(par),
                "W" => Move::West(par),
                "L" => Move::Rotate(par, -1),
                "R" => Move::Rotate(par, 1),
                "F" => Move::Forward(par),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(moves: &[Move]) -> f64 {
    //x,y
    let mut pos = (0.0, 0.0);
    let mut dir = 0.0;

    for m in moves {
        match m {
            Move::North(par) => pos.1 -= par,
            Move::South(par) => pos.1 += par,
            Move::East(par) => pos.0 += par,
            Move::West(par) => pos.0 -= par,
            Move::Rotate(par, d) => dir += *d as f64 * par.to_radians(),
            Move::Forward(par) => {
                pos.0 += (dir.cos() * par).round();
                pos.1 += (dir.sin() * par).round();
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
fn part2(moves: &[Move]) -> f64 {
    //x,y
    let mut way_pos: (f64, f64) = (10.0, 1.0);
    let mut ship_pos: (f64, f64) = (0.0, 0.0);

    for m in moves {
        match m {
            Move::North(par) => way_pos.1 += par,
            Move::South(par) => way_pos.1 -= par,
            Move::East(par) => way_pos.0 += par,
            Move::West(par) => way_pos.0 -= par,
            Move::Rotate(par, d) => {
                let par = -*d as f64 * par.to_radians();
                way_pos = (
                    way_pos.0 * par.cos() - way_pos.1 * par.sin(),
                    way_pos.1 * par.cos() + way_pos.0 * par.sin(),
                );
            }
            Move::Forward(par) => {
                ship_pos.0 += way_pos.0 * par;
                ship_pos.1 += way_pos.1 * par;
            }
        }
    }

    ship_pos.0.round().abs() + ship_pos.1.round().abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
F10
N3
F7
R90
F11";
        assert_eq!(part1(&parse_input(input)), 25.0);
    }

    #[test]
    fn test_2_1() {
        let input = "\
F10
N3
F7
R90
L90
R90
F11";
        assert_eq!(part2(&parse_input(input)), 286.0);
    }
}
