use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> (Vec<Vec<Seat>>, usize, usize) {
    (
        input
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| match c {
                        '.' => Seat::Floor,
                        'L' => Seat::Empty,
                        '#' => Seat::Occupied,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<Seat>>()
            })
            .collect(),
        input.lines().next().unwrap().len(),
        input.lines().count(),
    )
}

#[aoc(day11, part1)]
fn part1(input: &(Vec<Vec<Seat>>, usize, usize)) -> usize {
    let mut seats = input.0.clone();
    let w = input.1;
    let h = input.2;
    let mut changed;
    loop {
        changed = false;
        let mut tmp_seats = seats.clone();
        for y in 0..h {
            for x in 0..w {
                if seats[y][x] != Seat::Floor {
                    let occupied = count_visible_occupied(&seats, (x, y), (w, h), 1);

                    if seats[y][x] == Seat::Empty && occupied == 0 {
                        tmp_seats[y][x] = Seat::Occupied;
                        changed = true;
                    } else if seats[y][x] == Seat::Occupied && occupied >= 4 {
                        tmp_seats[y][x] = Seat::Empty;
                        changed = true;
                    }
                }
            }
        }
        seats = tmp_seats;
        if !changed {
            break;
        }
    }

    seats
        .iter()
        .flatten()
        .filter(|&&s| s == Seat::Occupied)
        .count()
}

#[aoc(day11, part2)]
fn part2(input: &(Vec<Vec<Seat>>, usize, usize)) -> usize {
    let mut seats = input.0.clone();
    let w = input.1;
    let h = input.2;
    let mut changed;

    loop {
        changed = false;
        let mut tmp_seats = seats.clone();
        for y in 0..h {
            for x in 0..w {
                if seats[y][x] != Seat::Floor {
                    let occupied = count_visible_occupied(&seats, (x, y), (w, h), -1);

                    if seats[y][x] == Seat::Empty && occupied == 0 {
                        tmp_seats[y][x] = Seat::Occupied;
                        changed = true;
                    } else if seats[y][x] == Seat::Occupied && occupied >= 5 {
                        tmp_seats[y][x] = Seat::Empty;
                        changed = true;
                    }
                }
            }
        }
        seats = tmp_seats;
        if !changed {
            break;
        }
    }

    seats
        .iter()
        .flatten()
        .filter(|&&s| s == Seat::Occupied)
        .count()
}

fn count_visible_occupied(
    seats: &[Vec<Seat>],
    index: (usize, usize),
    size: (usize, usize),
    r: i32,
) -> i32 {
    let dirs = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let w = size.0 as i32;
    let h = size.1 as i32;
    let mut occupied = 0_i32;

    for dir in dirs {
        let mut pos_x = index.0 as i32;
        let mut pos_y = index.1 as i32;

        let mut i = 0;

        pos_x += dir.0;
        pos_y += dir.1;

        let mut found = false;

        while pos_x >= 0 && pos_x < w && pos_y >= 0 && pos_y < h && !found && (r == -1 || i < r) {
            match seats[pos_y as usize][pos_x as usize] {
                Seat::Occupied => {
                    occupied += 1;
                    found = true;
                }
                Seat::Empty => {
                    found = true;
                }
                Seat::Floor => (),
            }

            pos_x += dir.0;
            pos_y += dir.1;
            i += 1;
        }
    }

    occupied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let input = "\
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        assert_eq!(part1(&parse_input(input)), 37);
    }

    #[test]
    fn test_2_1() {
        let input = "\
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        assert_eq!(part2(&parse_input(input)), 26);
    }
}
