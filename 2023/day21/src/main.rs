use pathfinding::{directed::bfs::bfs_reach, matrix::Matrix};
use std::collections::HashSet;

fn part1(input: &str, steps: usize) -> usize {
    let grid = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, c)| c.iter().position(|c| *c == 'S').map(|c| (r, c)))
        .unwrap();

    let result = bfs_reach((start, steps), |&(pos, steps)| {
        if steps == 0 {
            return vec![];
        }
        grid.neighbours(pos, false)
            .filter(|pos| grid.get(*pos) == Some(&'.') || grid.get(*pos) == Some(&'S'))
            .map(|pos| (pos, steps - 1))
            .collect::<Vec<_>>()
    });
    result.filter(|(_, steps)| *steps == 0).count()
}

fn part2(input: &str, steps: usize) -> usize {
    let grid = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let start = grid
        .items()
        .find_map(|(pos, c)| (*c == 'S').then_some(pos))
        .unwrap();
    let mut visited = HashSet::new();
    visited.insert((start.0 as isize, start.1 as isize));
    let mut values = [0; 3];
    for step in 1..=steps {
        let mut set = HashSet::with_capacity(4 * visited.len());
        for &(x, y) in &visited {
            set.extend(
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .filter(|(x, y)| grid[grid.constrain((*x, *y))] != '#'),
            );
        }
        visited = set;
        if step % grid.columns == steps % grid.columns {
            let index = step / grid.columns;
            values[index] = visited.len();
            if index == values.len() - 1 {
                break;
            }
        }
    }
    let b0 = values[0];
    let b1 = values[1] - values[0];
    let b2 = values[2] - values[1];
    let n = steps / grid.columns;
    b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input, 64));
    println!("Part 2: {}", part2(input, 26501365));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    const REALINPUT: &str = include_str!("../input.txt");
    #[test]
    fn part1_6() {
        assert_eq!(super::part1(INPUT, 6), 16);
    }
    #[test]
    fn part1_real() {
        assert_eq!(super::part1(REALINPUT, 64), 3594);
    }
    #[test]
    fn part2_real() {
        assert_eq!(super::part2(REALINPUT, 26501365), 605247138198755);
    }
}
