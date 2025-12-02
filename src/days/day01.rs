use crate::{DayResult, IntoDayResult};
use anyhow::Result;

pub fn solve(input: &str) -> Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut pos: i32 = 50;
    let mut dir: i32 = -1;
    let mut number: i32 = 0;
    let mut has_number = false;

    for &b in input.as_bytes() {
        match b {
            b'L' => dir = -1,
            b'R' => dir = 1,
            b'\n' => {
                if has_number {
                    apply_turn(&mut pos, dir, number, &mut p1, &mut p2);
                    number = 0;
                    has_number = false;
                }
            }
            b'0'..=b'9' => {
                number = number * 10 + (b - b'0') as i32;
                has_number = true;
            }
            _ => {}
        }
    }

    if has_number {
        apply_turn(&mut pos, dir, number, &mut p1, &mut p2);
    }

    (p1, p2).into_result()
}

#[inline(always)]
fn apply_turn(pos: &mut i32, dir: i32, steps: i32, p1: &mut i32, p2: &mut i32) {
    let steps_until_zero = if dir > 0 {
        if *pos == 0 {
            100
        } else {
            100 - *pos
        }
    } else if *pos == 0 {
        100
    } else {
        *pos
    };

    if steps >= steps_until_zero {
        *p2 += 1 + (steps - steps_until_zero) / 100;
    }

    *pos += dir * steps;
    *pos %= 100;
    if *pos < 0 {
        *pos += 100;
    }

    if *pos == 0 {
        *p1 += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((3, 6).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2025_", "day01", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_177, 6_768).into_day_result(), solution);
    }
}
