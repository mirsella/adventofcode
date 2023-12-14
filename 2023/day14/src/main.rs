use std::collections::HashMap;

use grid::Grid;

fn slide_up(col: &mut [&mut char]) {
    let mut hard: isize = -1;
    let mut last: isize = 0;
    for i in 0..col.len() {
        match col[i] {
            '#' => {
                hard = i as isize;
                last = 0;
            }
            'O' => {
                *col[i] = '.';
                *col[(hard + last + 1) as usize] = 'O';
                last += 1;
            }
            _ => (),
        }
    }
}

fn calculate_weight<'a>(col: impl Iterator<Item = &'a char>, row_len: usize) -> usize {
    col.enumerate().fold(
        0,
        |acc, (i, &c)| {
            if c == 'O' {
                acc + (row_len - i)
            } else {
                acc
            }
        },
    )
}

fn part1(input: &str) -> usize {
    let vec = input.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
    let mut grid = Grid::from_vec(vec, input.find('\n').unwrap());
    (0..grid.cols())
        .map(|i| {
            let mut col = grid.iter_col_mut(i).collect::<Vec<_>>();
            slide_up(col.as_mut_slice());
            calculate_weight(grid.iter_col(i), grid.cols())
        })
        .sum()
}
fn part2(input: &str, cycle: usize) -> usize {
    let vec = input.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
    let mut grid = Grid::from_vec(vec, input.find('\n').unwrap());
    let mut map = HashMap::with_capacity(1000);
    let mut count = 0;
    let (spins, cycle_len) = loop {
        count += 1;
        for _ in 0..4 {
            // FIX: 99% performance loss on this loop
            (0..grid.cols())
                .for_each(|i| slide_up(grid.iter_col_mut(i).collect::<Vec<_>>().as_mut_slice()));
            grid.rotate_right();
        }
        if let Some(dup) = map.insert(grid.iter().copied().collect::<Vec<_>>(), count) {
            break (count, count - dup);
        }
    };
    let spins_left = (cycle - spins) % cycle_len;
    for _ in 0..spins_left {
        for _ in 0..4 {
            (0..grid.cols())
                .for_each(|i| slide_up(grid.iter_col_mut(i).collect::<Vec<_>>().as_mut_slice()));
            grid.rotate_right();
        }
    }
    (0..grid.cols())
        .map(|i| calculate_weight(grid.iter_col(i), grid.cols()))
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000000));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 136);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT, 1000000000), 64);
    }
}
