use crate::{DayResult, IntoDayResult};
use anyhow::Result;
use std::ptr;

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
        let digits = unsafe { chunk.get_unchecked(..WIDTH) };
        p1 += best_two(digits) as usize;
        p2 += best_twelve::<WIDTH>(digits) as usize;
    }
    (p1, p2)
}

fn best_two(digits: &[u8]) -> u64 {
    // Track the best possible second digit while scanning from right to left so we only
    // touch the slice once and still respect ordering constraints.
    let n = digits.len();
    let mut best_suffix = unsafe { *digits.get_unchecked(n - 1) };
    let mut best_pair = 0u64;

    for i in (0..n - 1).rev() {
        let d1 = unsafe { *digits.get_unchecked(i) };
        let candidate = ((d1 - b'0') as u64) * 10 + ((best_suffix - b'0') as u64);
        if candidate > best_pair {
            best_pair = candidate;
            if best_pair == 99 {
                break;
            }
        }
        if d1 > best_suffix {
            best_suffix = d1;
        }
    }

    best_pair
}

fn best_twelve<const MAX_DIGITS: usize>(digits: &[u8]) -> u64 {
    if MAX_DIGITS <= 24 {
        return best_twelve_simple::<MAX_DIGITS>(digits);
    }
    best_twelve_tail_copy::<MAX_DIGITS>(digits)
}

#[inline(always)]
fn best_twelve_simple<const MAX_DIGITS: usize>(digits: &[u8]) -> u64 {
    let mut stack = [0u8; MAX_DIGITS];
    let mut len = 0;
    let mut to_remove = digits.len() - 12;
    for &digit in digits {
        while to_remove > 0 && len > 0 && digit > unsafe { *stack.get_unchecked(len - 1) } {
            len -= 1;
            to_remove -= 1;
        }
        unsafe { *stack.get_unchecked_mut(len) = digit };
        len += 1;
    }

    while to_remove > 0 {
        len -= 1;
        to_remove -= 1;
    }

    digits12_to_u64(&stack)
}

#[inline(always)]
fn best_twelve_tail_copy<const MAX_DIGITS: usize>(digits: &[u8]) -> u64 {
    let mut stack = [0u8; MAX_DIGITS];
    let mut len = 0;
    let mut to_remove = digits.len() - 12;
    let mut idx = 0;

    while idx < digits.len() {
        let digit = unsafe { *digits.get_unchecked(idx) };
        idx += 1;

        while to_remove > 0 && len > 0 && digit > unsafe { *stack.get_unchecked(len - 1) } {
            len -= 1;
            to_remove -= 1;
        }

        unsafe { *stack.get_unchecked_mut(len) = digit };
        len += 1;

        if to_remove == 0 {
            let remaining = digits.len() - idx;
            if remaining > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(
                        digits.get_unchecked(idx),
                        stack.get_unchecked_mut(len),
                        remaining,
                    );
                }
                len += remaining;
            }
            break;
        }
    }

    while to_remove > 0 {
        len -= 1;
        to_remove -= 1;
    }

    digits12_to_u64(&stack)
}

#[inline(always)]
fn digits12_to_u64(buf: &[u8]) -> u64 {
    let mut res = 0u64;
    for i in 0..12 {
        res = res * 10 + unsafe { (*buf.get_unchecked(i) - b'0') as u64 };
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
