use grid::Grid;

fn find_parallel(vec: &Vec<String>) -> Option<usize> {
    for i in 1..vec.len() {
        let mut valid = true;
        let mut l = i as isize - 1;
        let mut r = i;
        while l >= 0 && r < vec.len() {
            if vec[l as usize] != vec[r] {
                valid = false;
                break;
            }
            l -= 1;
            r += 1;
        }
        if valid {
            return Some(i);
        }
    }
    None
}
fn find_parallel_diff(vec: &Vec<String>, diff: usize) -> Option<usize> {
    let count_char_diff =
        |a: &str, b: &str| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count();
    for i in 1..vec.len() {
        let mut d = 0;
        let mut l = i as isize - 1;
        let mut r = i;
        while l >= 0 && r < vec.len() && d <= diff {
            d += count_char_diff(&vec[l as usize], &vec[r]);
            l -= 1;
            r += 1;
        }
        if d == diff {
            return Some(i);
        }
    }
    None
}
fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grid| {
            let chars = grid.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
            let grid = Grid::from_vec(chars, grid.find('\n').unwrap());
            let p = find_parallel(&grid.iter_rows().map(|i| i.collect::<String>()).collect());
            if let Some(i) = p {
                return i * 100;
            }
            let p = find_parallel(&grid.iter_cols().map(|i| i.collect::<String>()).collect());
            if let Some(i) = p {
                return i;
            }
            unreachable!()
        })
        .sum()
}
fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grid| {
            let chars = grid.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
            let grid = Grid::from_vec(chars, grid.find('\n').unwrap());
            let p = find_parallel_diff(
                &grid.iter_rows().map(|i| i.collect::<String>()).collect(),
                1,
            );
            if let Some(i) = p {
                return i * 100;
            }
            let p = find_parallel_diff(
                &grid.iter_cols().map(|i| i.collect::<String>()).collect(),
                1,
            );
            if let Some(i) = p {
                return i;
            }
            unreachable!()
        })
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 405);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 400);
    }
}
