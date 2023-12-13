use grid::Grid;

fn find_parallel(iter: impl Iterator<Item = impl PartialEq>) -> Option<usize> {
    let vec: Vec<_> = iter.into_iter().collect();
    (1..vec.len()).find(|&i| {
        (0..i)
            .rev()
            .zip(i..vec.len())
            .all(|(l, r)| vec[l] == vec[r])
    })
}

fn find_parallel_diff(
    iter: impl Iterator<Item = impl AsRef<str> + PartialEq>,
    diff: usize,
) -> Option<usize> {
    let vec: Vec<_> = iter.into_iter().collect();
    let count_char_diff =
        |a: &str, b: &str| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count();

    for i in 1..vec.len() {
        let mut d = 0;
        let mut l = i as isize - 1;
        let mut r = i;

        while l >= 0 && r < vec.len() && d <= diff {
            d += count_char_diff(vec[l as usize].as_ref(), vec[r].as_ref());
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
            if let Some(i) = find_parallel(grid.iter_rows().map(|i| i.collect::<String>())) {
                i * 100
            } else if let Some(i) = find_parallel(grid.iter_cols().map(|i| i.collect::<String>())) {
                i
            } else {
                unreachable!()
            }
        })
        .sum()
}
fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grid| {
            let chars = grid.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
            let grid = Grid::from_vec(chars, grid.find('\n').unwrap());
            if let Some(i) = find_parallel_diff(grid.iter_rows().map(|i| i.collect::<String>()), 1)
            {
                i * 100
            } else if let Some(i) =
                find_parallel_diff(grid.iter_cols().map(|i| i.collect::<String>()), 1)
            {
                i
            } else {
                unreachable!()
            }
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
