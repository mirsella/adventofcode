use itertools::Itertools;
use pathfinding::matrix::{directions::*, Matrix};
use rayon::prelude::*;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let directions = [(N, '^'), (E, '>'), (S, 'v'), (W, '<')];
    let get_direction = |char: char| directions.iter().find(|(_, c)| char == *c).unwrap().0;
    let get_next_direction =
        |char: char| directions[(directions.iter().position(|v| char == v.1).unwrap() + 1) % 4];
    let mut m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut pos = m
        .items()
        .find_map(|(p, c)| directions.iter().any(|dir| dir.1 == *c).then_some(p))
        .unwrap();
    loop {
        let Some((newpos, new)) = m
            .move_in_direction(pos, get_direction(m[pos]))
            .map(|p| (p, m[p]))
        else {
            break;
        };
        if new == '#' {
            m[pos] = get_next_direction(m[pos]).1
        } else {
            m[newpos] = m[pos];
            pos = newpos;
        }
    }
    m.values()
        .filter(|&&c| directions.iter().any(|dir| dir.1 == c))
        .count()
}

// try to do it without bruteforce
fn _part2_clever(input: &str) -> usize {
    let directions = [(N, '^'), (E, '>'), (S, 'v'), (W, '<')];
    let get_direction = |char: char| directions.iter().find(|(_, c)| char == *c).unwrap().0;
    let get_next_direction =
        |char: char| directions[(directions.iter().position(|v| char == v.1).unwrap() + 1) % 4];
    let mut m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut pos = m
        .items()
        .find_map(|(p, c)| directions.iter().any(|dir| dir.1 == *c).then_some(p))
        .unwrap();
    let start = pos;
    let mut possible_obstructions = HashSet::new();
    loop {
        let direction = get_direction(m[pos]);
        let Some((newpos, new)) = m.move_in_direction(pos, direction).map(|p| (p, m[p])) else {
            break;
        };
        let next_direction = get_next_direction(m[pos]);
        if new == '#' {
            m[pos] = next_direction.1;
        } else {
            m[newpos] = m[pos];
            pos = newpos;
            // if putting a obstruction at the new tile will make the guard join a path he already took
            let next_next_direction = get_next_direction(next_direction.1);
            for tile in m.in_direction(pos, get_next_direction(m[pos]).0) {
                if m[tile] == next_direction.1
                    || (m[tile] == next_next_direction.1
                        && m[m.move_in_direction(tile, next_direction.0).unwrap()] == '#')
                {
                    let move_in_direction = m.move_in_direction(pos, direction).unwrap();
                    if move_in_direction != start {
                        possible_obstructions.insert(move_in_direction);
                    }
                    break;
                }
            }
        }
    }
    for pos in possible_obstructions.iter() {
        m[*pos] = 'O'
    }
    for row in m.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    possible_obstructions.len()
}

// bruteforce but thanks to rayon its still ~250ms to run
fn part2(input: &str) -> usize {
    let directions = [(N, '^'), (E, '>'), (S, 'v'), (W, '<')];
    let get_direction = |char: char| directions.iter().find(|(_, c)| char == *c).unwrap().0;
    let get_next_direction =
        |char: char| directions[(directions.iter().position(|v| char == v.1).unwrap() + 1) % 4];
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let pos = m
        .items()
        .find_map(|(p, c)| directions.iter().any(|dir| dir.1 == *c).then_some(p))
        .unwrap();
    let is_infinite = |mut m: Matrix<char>, mut pos: (usize, usize)| -> bool {
        let mut seen = HashSet::new();
        loop {
            let direction = get_direction(m[pos]);
            let Some((newpos, new)) = m.move_in_direction(pos, direction).map(|p| (p, m[p])) else {
                break;
            };
            let next_direction = get_next_direction(m[pos]);
            if new == '#' {
                m[pos] = next_direction.1;
            } else {
                m[newpos] = m[pos];
                pos = newpos;
                if !seen.insert((pos, m[pos])) {
                    return true;
                }
            }
        }
        false
    };
    m.keys()
        .collect_vec()
        .par_iter()
        .filter(|&&tile| {
            if tile == pos {
                return false;
            }
            let mut m = m.clone();
            m[tile] = '#';
            is_infinite(m, pos)
        })
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 41);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 6);
    }
}
