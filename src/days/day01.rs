use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};

pub fn solve(input: &str) -> Result<DayResult> {
    let mut pos: isize = 50;
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let dir = line.as_bytes()[0];
        let number = line[1..]
            .parse::<isize>()
            .context("failed to parse integer")?;

        p2 += zero_hits(pos, dir, number);

        pos = match dir {
            b'L' => (pos - number).rem_euclid(100),
            b'R' => (pos + number).rem_euclid(100),
            _ => unreachable!(),
        };

        if pos == 0 {
            p1 += 1;
        }
    }

    (p1, p2).into_result()
}

fn zero_hits(pos: isize, dir: u8, steps: isize) -> isize {
    let steps_until_zero = match dir {
        b'R' => {
            if pos == 0 {
                100
            } else {
                100 - pos
            }
        }
        b'L' => {
            if pos == 0 {
                100
            } else {
                pos
            }
        }
        _ => unreachable!(),
    };

    if steps < steps_until_zero {
        0
    } else {
        1 + (steps - steps_until_zero) / 100
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, IntoDayResult};

    #[ignore]
    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((3, 6).into_day_result(), solution);
    }

    #[ignore]
    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2025_", "day01", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_177, 6_768).into_day_result(), solution);
    }
}
