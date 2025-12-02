use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};

pub fn solve(_input: &str) -> Result<DayResult> {
    let input = _input.trim();

    let mut p1 = 0;
    let mut p2 = 0;

    for ids in input.split(',') {
        let (a_str, b_str) = ids.split_once('-').context("no hyphen found")?;
        let a = a_str.parse::<u64>().context("failed to parse a")?;
        let b = b_str.parse::<u64>().context("failed to parse b")?;
        for id in a..=b {
            let mut buf = itoa::Buffer::new();
            let id_str = buf.format(id);

            if invalid(&id_str, 2) {
                p1 += id;
            }

            for repeats in (2..=id_str.len()).rev() {
                if invalid(&id_str, repeats) {
                    p2 += id;
                    break;
                }
            }
        }
    }

    (p1, p2).into_result()
}

fn invalid(id: &str, repeats: usize) -> bool {
    if id.len() % repeats != 0 {
        return false;
    }

    let bytes = id.as_bytes();
    let chunk_size = bytes.len() / repeats;
    let first = &bytes[..chunk_size];

    bytes.chunks_exact(chunk_size).all(|chunk| chunk == first)
}

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
        assert_eq!((32_976_912_643_usize, 54_446_379_122_usize).into_day_result(), solution);
    }
}
