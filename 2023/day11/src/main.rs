#[derive(PartialEq)]
struct Galaxy {
    row: usize,
    col: usize,
}
struct Grid(pub Vec<Vec<char>>);
impl Grid {
    fn parse(input: &str) -> Self {
        let data: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        Self(data)
    }
    fn expand_galaxy(&self, g: &mut Galaxy, factor: usize) {
        let mut row_expansion = 0;
        for row in 0..g.row {
            if !self.0[row].iter().any(|&c| c == '#') {
                row_expansion += 1
            }
        }
        let mut col_expansion = 0;
        for col in 0..g.col {
            if !(0..self.0.len()).any(|i| self.0[i][col] == '#') {
                col_expansion += 1;
            }
        }
        g.row += row_expansion * factor - row_expansion;
        g.col += col_expansion * factor - col_expansion;
    }
    fn get_galaxys(&self, expansion: usize) -> Vec<Galaxy> {
        let mut galaxys = Vec::new();
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                if self.0[row][col] == '#' {
                    let mut g = Galaxy { row, col };
                    self.expand_galaxy(&mut g, expansion);
                    galaxys.push(g);
                }
            }
        }
        galaxys
    }
}
fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    let galaxys = grid.get_galaxys(2);
    let mut count = 0;
    for galaxy in &galaxys {
        for other in galaxys.iter().skip_while(|&g| g != galaxy) {
            let shortest_path = (galaxy.row as isize - other.row as isize).abs()
                + (galaxy.col as isize - other.col as isize).abs();
            count += shortest_path as usize;
        }
    }
    count
}
fn part2(input: &str, expansion: usize) -> usize {
    let grid = Grid::parse(input);
    let galaxys = grid.get_galaxys(expansion);
    let mut count = 0;
    for galaxy in &galaxys {
        for other in galaxys.iter().skip_while(|&g| g != galaxy) {
            let shortest_path = (galaxy.row as isize - other.row as isize).abs()
                + (galaxy.col as isize - other.col as isize).abs();
            count += shortest_path as usize;
        }
    }
    count
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 374);
    }
    #[test]
    fn part2_1() {
        assert_eq!(super::part2(INPUT, 10), 1030);
    }
    #[test]
    fn part2_2() {
        assert_eq!(super::part2(INPUT, 100), 8410);
    }
}
