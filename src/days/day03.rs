use crate::{DayResult, IntoDayResult};
use anyhow::Result;

const TEST_MAX_DIGITS: usize = 15;
const REAL_MAX_DIGITS: usize = 100;

pub fn solve(_input: &str, is_test: bool) -> Result<DayResult> {
    let input = _input.as_bytes();
    let (p1, p2) = if is_test {
        solve_impl::<TEST_MAX_DIGITS>(input)
    } else {
        solve_impl::<REAL_MAX_DIGITS>(input)
    };

    (p1, p2).into_result()
}

fn solve_impl<const WIDTH: usize>(input: &[u8]) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;
    let stride = WIDTH + 1;
    for chunk in input.chunks_exact(stride) {
        let digits = &chunk[..WIDTH];
        p1 += best_two(digits) as usize;
        p2 += best_twelve::<WIDTH>(digits) as usize;
    }
    (p1, p2)
}

fn best_two(digits: &[u8]) -> u64 {
    let n = digits.len();
    let mut max_d1 = 0;
    let mut idx_d1 = 0;

    for (i, &d) in digits[..n - 1].iter().enumerate() {
        if d > max_d1 {
            max_d1 = d;
            idx_d1 = i;
            if max_d1 == b'9' {
                break;
            }
        }
    }

    let mut max_d2 = 0;
    for &d in &digits[idx_d1 + 1..] {
        if d > max_d2 {
            max_d2 = d;
            if max_d2 == b'9' {
                break;
            }
        }
    }

    ((max_d1 - b'0') as u64) * 10 + (max_d2 - b'0') as u64
}

fn best_twelve<const MAX_DIGITS: usize>(digits: &[u8]) -> u64 {
    let mut stack = [0u8; MAX_DIGITS];
    let mut len = 0;
    let mut to_remove = digits.len() - 12;

    for &digit in digits {
        while to_remove > 0 && len > 0 && digit > stack[len - 1] {
            len -= 1;
            to_remove -= 1;
        }
        stack[len] = digit;
        len += 1;
    }

    let mut res = 0u64;
    for i in 0..12 {
        res = res * 10 + (stack[i] - b'0') as u64;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::{days::day03::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day03.txt");
        let solution = solve(INPUT, true).unwrap();
        assert_eq!((357, 3_121_910_778_619_usize).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2025_", "day03", ".txt"));
        let solution = solve(INPUT, false).unwrap();
        assert_eq!(
            (17_196, 171_039_099_596_062_usize).into_day_result(),
            solution
        );
    }
}
