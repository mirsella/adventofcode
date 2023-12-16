use grid::Grid;
use itertools::Itertools;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/// x, y
impl From<Direction> for (isize, isize) {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn _print_energized_grid(grid: &Grid<char>, energized: &BTreeSet<(isize, isize, Direction)>) {
    let mut grid = grid.clone();
    for (x, y, _) in energized {
        *grid.get_mut(*y as usize, *x as usize).unwrap() = '#';
    }
    for row in grid.iter_rows() {
        println!("{}", row.collect::<String>());
    }
    println!();
}

/// x, y current pos, direction the ray was going
fn fill_ray(
    grid: &Grid<char>,
    history: &mut BTreeSet<(isize, isize, Direction)>,
    x: isize,
    y: isize,
    direction: Direction,
) {
    let c = match grid.get(y as usize, x as usize) {
        Some(c) => c,
        None => return,
    };
    let (dx, dy) = direction.into();
    if !history.insert((x, y, direction)) {
        return;
    }
    match c {
        '|' if matches!(direction, Direction::Left | Direction::Right) => {
            fill_ray(grid, history, x, y - 1, Direction::Up);
            fill_ray(grid, history, x, y + 1, Direction::Down);
        }
        '-' if matches!(direction, Direction::Up | Direction::Down) => {
            fill_ray(grid, history, x - 1, y, Direction::Left);
            fill_ray(grid, history, x + 1, y, Direction::Right);
        }
        '/' => match direction {
            Direction::Up => fill_ray(grid, history, x + 1, y, Direction::Right),
            Direction::Down => fill_ray(grid, history, x - 1, y, Direction::Left),
            Direction::Left => fill_ray(grid, history, x, y + 1, Direction::Down),
            Direction::Right => fill_ray(grid, history, x, y - 1, Direction::Up),
        },
        '\\' => match direction {
            Direction::Down => fill_ray(grid, history, x + 1, y, Direction::Right),
            Direction::Up => fill_ray(grid, history, x - 1, y, Direction::Left),
            Direction::Right => fill_ray(grid, history, x, y + 1, Direction::Down),
            Direction::Left => fill_ray(grid, history, x, y - 1, Direction::Up),
        },
        _ => fill_ray(grid, history, x + dx, y + dy, direction),
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_vec(
        input.chars().filter(|&c| c != '\n').collect(),
        input.find('\n').unwrap(),
    );
    let mut energized_tiles = BTreeSet::new();
    fill_ray(&grid, &mut energized_tiles, 0, 0, Direction::Right);
    // print_energized_grid(&grid, &energized_tiles);
    energized_tiles
        .into_iter()
        .unique_by(|&(x, y, _)| (x, y))
        .count()
}
fn part2(input: &str) -> usize {
    let grid = Grid::from_vec(
        input.chars().filter(|&c| c != '\n').collect(),
        input.find('\n').unwrap(),
    );
    let grid = Arc::new(grid);
    let starts = [
        (0..grid.cols() - 1, 0..1, Direction::Down),
        (
            0..grid.cols() - 1,
            (grid.cols() - 1)..grid.cols(),
            Direction::Up,
        ),
        (0..1, 0..grid.rows() - 1, Direction::Right),
        (
            (grid.cols() - 1)..grid.cols(),
            0..grid.rows() - 1,
            Direction::Left,
        ),
    ];
    let mut counts = Vec::with_capacity(grid.rows() * 2 + grid.cols() * 2);
    for (x_range, y_range, direction) in starts.iter() {
        for x in x_range.clone() {
            for y in y_range.clone() {
                let grid = grid.clone();
                let direction = *direction;
                counts.push(std::thread::spawn(move || {
                    let mut energized_tiles = BTreeSet::new();
                    fill_ray(
                        &grid,
                        &mut energized_tiles,
                        x as isize,
                        y as isize,
                        direction,
                    );
                    energized_tiles
                        .iter()
                        .unique_by(|&(x, y, _)| (x, y))
                        .count()
                }))
            }
        }
    }
    counts.into_iter().map(|h| h.join().unwrap()).max().unwrap()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 46);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 51);
    }
}
