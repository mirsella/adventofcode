use pathfinding::{directed::bfs::bfs_reach, matrix::Matrix};
use std::collections::HashSet;

fn part1(input: &str, steps: usize) -> usize {
    let matrix = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let start = matrix
        .iter()
        .enumerate()
        .find_map(|(r, c)| c.iter().position(|c| *c == 'S').map(|c| (r, c)))
        .unwrap();

    let result = bfs_reach((start, steps), |&(pos, steps)| {
        if steps == 0 {
            return vec![];
        }
        matrix
            .neighbours(pos, false)
            .filter(|pos| matrix.get(*pos) == Some(&'.') || matrix.get(*pos) == Some(&'S'))
            .map(|pos| (pos, steps - 1))
            .collect::<Vec<_>>()
    });
    result.filter(|(_, steps)| *steps == 0).count()
}

fn part2(input: &str, steps: usize) -> usize {
    let matrix = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let start = matrix
        .iter()
        .enumerate()
        .find_map(|(r, c)| {
            c.iter()
                .position(|c| *c == 'S')
                .map(|c| (r as isize, c as isize))
        })
        .unwrap();
    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut values = [0; 3];
    for step in 0..steps {
        let mut set = HashSet::new();
        for (x, y) in &visited {
            set.extend(
                neighbors
                    .iter()
                    .map(|(dx, dy)| (*x + dx, *y + dy))
                    .filter(|(x, y)| {
                        matrix
                            .get((
                                (x.rem_euclid(matrix.columns as isize)) as usize,
                                (y.rem_euclid(matrix.rows as isize)) as usize,
                            ))
                            .expect("should always be inbound")
                            != &'#'
                    }),
            );
        }
        if step % matrix.columns == steps % matrix.columns {
            let index = step / matrix.columns;
            values[index] = visited.len();
            if index == values.len() - 1 {
                break;
            }
        }
        visited = set;
    }
    let b0 = values[0];
    let b1 = values[1] - values[0];
    let b2 = values[2] - values[1];
    let n = steps / matrix.columns;
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
    #[test]
    fn part1_6() {
        assert_eq!(super::part1(INPUT, 6), 16);
    }
    #[test]
    fn part2_10() {
        assert_eq!(super::part2(INPUT, 10), 50);
    }
    #[test]
    fn part2_50() {
        assert_eq!(super::part2(INPUT, 50), 1594);
    }
    #[test]
    fn part2_100() {
        assert_eq!(super::part2(INPUT, 100), 6536);
    }
    #[test]
    fn part2_500() {
        assert_eq!(super::part2(INPUT, 500), 167004);
    }
    #[test]
    fn part2_1000() {
        assert_eq!(super::part2(INPUT, 1000), 668697);
    }
    #[test]
    fn part2_5000() {
        assert_eq!(super::part2(INPUT, 5000), 16733044);
    }
}
