use grid::Grid;

const DIRECTIONS: [(isize, isize, char); 4] =
    [(0, 1, 'v'), (1, 0, '>'), (0, -1, '^'), (-1, 0, '<')];

fn dfs(grid: &mut Grid<char>, (x, y): (isize, isize), path: usize, paths: &mut Vec<usize>) {
    // found the end
    if y == grid.rows() as isize - 1 {
        paths.push(path);
        return;
    }

    for (dx, dy, c) in DIRECTIONS.iter() {
        let nx = x + dx;
        let ny = y + dy;
        match grid.get(ny, nx).copied() {
            Some(before) if before == '.' || before == *c => {
                // print_grid(grid);
                *grid.get_mut(ny, nx).unwrap() = 'O';
                dfs(grid, (nx, ny), path + 1, paths);
                *grid.get_mut(ny, nx).unwrap() = before;
            }
            _ => (),
        }
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from_vec(
        input.replace('\n', "").chars().collect(),
        input.find('\n').unwrap(),
    );
    let start_x = grid.iter_row(0).position(|&c| c == '.').unwrap();
    let mut result = Vec::new();
    dfs(&mut grid, (0, start_x as isize), 0, &mut result);
    *result.iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_vec(
        input
            .replace('\n', "")
            .replace(['<', '>', '^', 'v'], ".")
            .chars()
            .collect(),
        input.find('\n').unwrap(),
    );
    let start_x = grid.iter_row(0).position(|&c| c == '.').unwrap();
    let mut result = Vec::new();
    dfs(&mut grid, (0, start_x as isize), 0, &mut result);
    *result.iter().max().unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 94);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 154);
    }
}
