use crate::{DayResult, IntoDayResult};
use anyhow::Result;

pub fn solve(_input: &str) -> Result<DayResult> {
    let mut board: Vec<Vec<bool>> = _input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();
    let mut removeable = vec![];

    run_loop(&board, &mut removeable);

    let p1 = removeable.len();
    let mut p2 = removeable.len();

    while removeable.len() > 0 {
        for (x, y) in removeable.drain(..) {
            board[x][y] = false;
        }
        run_loop(&board, &mut removeable);
        p2 += removeable.len();
    }


    (p1, p2).into_result()
}

fn run_loop(board: &Vec<Vec<bool>>, removeable: &mut Vec<(usize, usize)>) {
        for x in 0..board.len() {
        for y in 0..board[0].len() {
            if !board[x][y] {
                continue;
            }
            let mut adjacent = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    let Ok(nx) = usize::try_from(nx) else {
                        continue;
                    };
                    if nx >= board.len() {
                        continue;
                    }
                    let Ok(ny) = usize::try_from(ny) else {
                        continue;
                    };
                    if ny >= board[0].len() {
                        continue;
                    }
                    if board[nx][ny] {
                        adjacent += 1;
                    }
                }
            }
            if adjacent < 4 {
                removeable.push((x, y));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day04::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day04.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((13, 43).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2025_", "day04", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_602, 9_518).into_day_result(), solution);
    }
}
