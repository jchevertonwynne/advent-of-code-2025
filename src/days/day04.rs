use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

use crate::{DayResult, IntoDayResult};
use anyhow::Result;

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve(_input: &str) -> Result<DayResult> {
    let bytes = _input.as_bytes();
    let mut raw = Vec::with_capacity(bytes.len());
    let mut width = None;
    let mut current_width = 0usize;
    let mut height = 0usize;

    for &b in bytes {
        match b {
            b'@' => {
                raw.push(true);
                current_width += 1;
            }
            b'.' => {
                raw.push(false);
                current_width += 1;
            }
            b'\n' => {
                if current_width == 0 {
                    continue;
                }
                if let Some(w) = width {
                    debug_assert_eq!(w, current_width);
                } else {
                    width = Some(current_width);
                }
                height += 1;
                current_width = 0;
            }
            _ => {}
        }
    }

    if current_width > 0 {
        if let Some(w) = width {
            debug_assert_eq!(w, current_width);
        } else {
            width = Some(current_width);
        }
        height += 1;
    }

    let width = width.expect("lines must not be empty");
    assert!(height > 0, "input must contain at least one row");
    debug_assert_eq!(raw.len(), width * height);

    let cells = raw.into_iter().map(Cell::new).collect();
    let mut board = Grid::new(height, width, cells);
    let rows = board.height();
    let cols = board.width();
    let rows_i = rows as isize;
    let cols_i = cols as isize;
    let mut queue = VecDeque::new();

    for x in 0..rows {
        for y in 0..cols {
            if !board[(x, y)].alive {
                continue;
            }

            let adjacent = alive_neighbor_count(&board, x, y, rows_i, cols_i);
            {
                let cell = board.get_mut(x, y);
                cell.degree = adjacent;
                if adjacent < 4 {
                    cell.queued = true;
                    queue.push_back((x, y));
                }
            }
        }
    }

    let p1 = queue.len();
    let mut removed = 0usize;

    // Peel low-degree tiles using a queue so we only revisit cells whose degree drops.
    while let Some((x, y)) = queue.pop_front() {
        {
            let cell = board.get_mut(x, y);
            if !cell.alive {
                cell.queued = false;
                continue;
            }
            cell.alive = false;
            cell.queued = false;
        }
        removed += 1;

        for &(dx, dy) in &NEIGHBOR_OFFSETS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= rows_i || ny < 0 || ny >= cols_i {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            let neighbor = board.get_mut(nx, ny);
            if !neighbor.alive || neighbor.degree == 0 {
                continue;
            }
            neighbor.degree -= 1;
            if neighbor.degree == 3 && !neighbor.queued {
                neighbor.queued = true;
                queue.push_back((nx, ny));
            }
        }
    }

    (p1, removed).into_result()
}

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
    degree: u8,
    queued: bool,
}

impl Cell {
    fn new(alive: bool) -> Self {
        Self {
            alive,
            degree: 0,
            queued: false,
        }
    }
}

#[derive(Clone)]
struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn new(height: usize, width: usize, data: Vec<T>) -> Self {
        debug_assert_eq!(height * width, data.len());
        Self {
            data,
            height,
            width,
        }
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x * self.width + y
    }

    fn get(&self, x: usize, y: usize) -> &T {
        let idx = self.idx(x, y);
        unsafe { self.data.get_unchecked(idx) }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let idx = self.idx(x, y);
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y)
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
    }
}

fn alive_neighbor_count(board: &Grid<Cell>, x: usize, y: usize, rows: isize, cols: isize) -> u8 {
    let xi = x as isize;
    let yi = y as isize;
    let mut adjacent = 0u8;
    for &(dx, dy) in &NEIGHBOR_OFFSETS {
        let nx = xi + dx;
        let ny = yi + dy;
        if nx < 0 || nx >= rows || ny < 0 || ny >= cols {
            continue;
        }
        if board[(nx as usize, ny as usize)].alive {
            adjacent += 1;
        }
    }
    adjacent
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
