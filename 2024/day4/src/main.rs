use pathfinding::matrix::{directions::*, Matrix};
use std::collections::HashSet;
use std::iter;

fn part1(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut set = HashSet::new();
    for tile in m.keys() {
        for direction in DIRECTIONS_8 {
            let line = iter::once(tile)
                .chain(m.in_direction(tile, direction))
                .map(|t| (t, m[t]))
                .collect::<Vec<_>>();
            let str = line.iter().map(|l| l.1).collect::<String>();
            for indice in str.match_indices("XMAS") {
                let tiles = (line[indice.0].0, line[indice.0 + 3].0);
                set.insert(tiles);
            }
        }
    }
    set.len()
}
fn part2(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut set = HashSet::new();
    for tile in m.keys() {
        for direction in [NE, NW, SE, SW] {
            let line = iter::once(tile)
                .chain(m.in_direction(tile, direction))
                .map(|t| (t, m[t]))
                .collect::<Vec<_>>();
            let str = line.iter().map(|l| l.1).collect::<String>();
            for indice in str.match_indices("MAS") {
                let tiles = (line[indice.0].0, line[indice.0 + 1].0, line[indice.0 + 2].0);
                set.insert(tiles);
            }
        }
    }
    set.iter()
        .filter(|tiles| set.iter().any(|t| t.1 == tiles.1 && t.0 != tiles.0))
        .count()
        / 2
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 18);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 9);
    }
}
