use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};
use bstr::ByteSlice;

pub fn solve(_input: &str) -> Result<DayResult> {
    let mut p1 = 0usize;
    let mut p2 = 0usize;
    let input = bstr::BStr::new(_input);

    for batteries in input.lines() {
        let digits = batteries.as_bytes();

        if digits.is_empty() {
            continue;
        }

        let best_two = max_number(digits, 2).context("line shorter than 2 digits")?;
        let best_twelve = max_number(digits, 12).context("line shorter than 12 digits")?;

        p1 += best_two as usize;
        p2 += best_twelve as usize;
    }

    (p1, p2).into_result()
}

fn max_number(digits: &[u8], pick: usize) -> Option<u64> {
    if digits.len() < pick {
        return None;
    }

    let mut start = 0usize;
    let mut remaining = pick;
    let mut result = 0u64;

    while remaining > 0 {
        let end = digits.len() - remaining;
        let mut max_digit = 0u8;
        let mut max_idx = 0usize;
        let mut found = false;

        for (idx, &byte) in digits[start..=end].iter().enumerate() {
            if !found || byte > max_digit {
                max_digit = byte;
                max_idx = idx;
                found = true;
                if max_digit == b'9' {
                    break;
                }
            }
        }

        if !found {
            return None;
        }

        result = result * 10 + (max_digit - b'0') as u64;
        start += max_idx + 1;
        remaining -= 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::{days::day03::solve, IntoDayResult};

    #[ignore]
    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day03.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((357, 3_121_910_778_619_usize).into_day_result(), solution);
    }

    #[ignore]
    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2025_", "day03", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (17_196, 171_039_099_596_062_usize).into_day_result(),
            solution
        );
    }
}
