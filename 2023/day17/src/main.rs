use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

fn pathfind(input: &str, min: usize, max: usize) -> usize {
    let grid = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap();

    let result = dijkstra(
        &((0, 0), (0, 0), 0),
        |&(pos, (direction_x, direction_y), consecutive_count)| {
            let mut moves = Vec::new();
            let mut add_moves = |direction, consecutive_count| {
                if let Some(new_pos) = grid.move_in_direction(pos, direction) {
                    moves.push(((new_pos, direction, consecutive_count), grid[new_pos]));
                };
            };
            if consecutive_count < max {
                add_moves((direction_x, direction_y), consecutive_count + 1);
            }
            if consecutive_count >= min {
                add_moves((-direction_y, -direction_x), 1);
                add_moves((direction_y, direction_x), 1);
            } else if consecutive_count == 0 {
                add_moves((1, 0), 1);
                add_moves((0, 1), 1);
            }
            moves
        },
        |pos| pos.0 == (grid.rows - 1, grid.columns - 1) && pos.2 >= min,
    )
    .unwrap();

    // print the grid with the path
    for (x, row) in grid.iter().enumerate() {
        for (y, digit) in row.iter().enumerate() {
            if result.0.iter().any(|p| p.0 == (x, y)) {
                print!("#");
            } else {
                print!("{}", digit);
            }
        }
        println!();
    }

    result.1 as usize
}

fn part1(input: &str) -> usize {
    pathfind(input, 1, 3)
}

fn part2(input: &str) -> usize {
    pathfind(input, 4, 10)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    const INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 102);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 94);
    }
    #[test]
    fn part2_2() {
        assert_eq!(super::part2(INPUT2), 71);
    }
}
