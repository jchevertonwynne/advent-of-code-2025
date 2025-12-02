use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};

pub fn solve(_input: &str) -> Result<DayResult> {
    let input = _input.trim();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut buf = itoa::Buffer::new();

    for ids in input.split(',') {
        let (a_str, b_str) = ids.split_once('-').context("no hyphen found")?;
        let a = a_str.parse::<u64>().context("failed to parse a")?;
        let b = b_str.parse::<u64>().context("failed to parse b")?;
        let mut id = a;
        while id <= b {
            let id_bytes = buf.format(id).as_bytes();
            let len = id_bytes.len();

            let mut matched = false;

            if id_bytes.len() > 1 && is_all_same(id_bytes) {
                matched = true;
                if len & 1 == 0 {
                    p1 += id;
                }
            } else {
                let halves_equal = len & 1 == 0 && halves_match(id_bytes);
                if halves_equal {
                    p1 += id;
                }

                for &repeats in repeat_factors(len) {
                    if repeats == len {
                        // chunk size == 1 already ruled out by all-same fast-path
                        continue;
                    }
                    if repeats == 2 {
                        if halves_equal {
                            matched = true;
                        }
                    } else if is_repeat(id_bytes, repeats) {
                        matched = true;
                    }

                    if matched {
                        break;
                    }
                }
            }

            if matched {
                p2 += id;
            }

            id += 1;
        }
    }

    (p1, p2).into_result()
}

fn halves_match(bytes: &[u8]) -> bool {
    let half = bytes.len() / 2;
    bytes[..half] == bytes[half..]
}

fn is_all_same(bytes: &[u8]) -> bool {
    bytes.windows(2).all(|w| w[0] == w[1])
}

fn repeat_factors(len: usize) -> &'static [usize] {
    match len {
        1 => &[],
        2 => &LEN_2,
        3 => &LEN_3,
        4 => &LEN_4,
        5 => &LEN_5,
        6 => &LEN_6,
        7 => &LEN_7,
        8 => &LEN_8,
        9 => &LEN_9,
        10 => &LEN_10,
        11 => &LEN_11,
        12 => &LEN_12,
        13 => &LEN_13,
        14 => &LEN_14,
        15 => &LEN_15,
        16 => &LEN_16,
        17 => &LEN_17,
        18 => &LEN_18,
        19 => &LEN_19,
        20 => &LEN_20,
        _ => &[],
    }
}

fn is_repeat(bytes: &[u8], repeats: usize) -> bool {
    let chunk_size = bytes.len() / repeats;
    let first = &bytes[..chunk_size];
    let mut chunks = bytes.chunks_exact(chunk_size);
    chunks.next();
    for chunk in chunks {
        if chunk != first {
            return false;
        }
    }

    true
}

const LEN_2: [usize; 1] = [2];
const LEN_3: [usize; 1] = [3];
const LEN_4: [usize; 2] = [4, 2];
const LEN_5: [usize; 1] = [5];
const LEN_6: [usize; 3] = [6, 3, 2];
const LEN_7: [usize; 1] = [7];
const LEN_8: [usize; 3] = [8, 4, 2];
const LEN_9: [usize; 2] = [9, 3];
const LEN_10: [usize; 3] = [10, 5, 2];
const LEN_11: [usize; 1] = [11];
const LEN_12: [usize; 5] = [12, 6, 4, 3, 2];
const LEN_13: [usize; 1] = [13];
const LEN_14: [usize; 3] = [14, 7, 2];
const LEN_15: [usize; 3] = [15, 5, 3];
const LEN_16: [usize; 4] = [16, 8, 4, 2];
const LEN_17: [usize; 1] = [17];
const LEN_18: [usize; 5] = [18, 9, 6, 3, 2];
const LEN_19: [usize; 1] = [19];
const LEN_20: [usize; 5] = [20, 10, 5, 4, 2];

#[cfg(test)]
mod tests {
    use crate::{days::day02::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day02.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (1_227_775_554_usize, 4_174_379_265_usize).into_day_result(),
            solution
        );
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2025_", "day02", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (32_976_912_643_usize, 54_446_379_122_usize).into_day_result(),
            solution
        );
    }
}
