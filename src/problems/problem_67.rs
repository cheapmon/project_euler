use super::problem_18::iterate_triangle;
use crate::util;

pub(super) fn run() -> u64 {
    let s = std::fs::read_to_string("resources/0067_triangle.txt").unwrap();
    let width = 100;

    let data = s.lines()
        .flat_map(|line| {
            let mut v = line.trim()
                .split(" ")
                .map(|part| part.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            v.resize(width, 0);
            v
        }).collect();
    let mut grid = util::Grid::new(data, width);

    iterate_triangle(&mut grid)
}