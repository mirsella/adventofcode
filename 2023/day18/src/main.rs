fn part1(input: &str) -> usize {
    let (last_point, mut area, perimeter) =
        input
            .lines()
            .fold(([0, 0], 0, 0), |([x0, y0], area, perimeter), line| {
                let mut s = line.split(' ');
                let dir = s.next().unwrap();
                let steps = s.next().unwrap().parse::<isize>().unwrap();

                let [x1, y1] = match dir {
                    "R" => [x0 + steps, y0],
                    "D" => [x0, y0 + steps],
                    "L" => [x0 - steps, y0],
                    "U" => [x0, y0 - steps],
                    _ => unreachable!(),
                };

                ([x1, y1], area + (y0 + y1) * (x1 - x0), perimeter + steps)
            });

    area += last_point[1] * (0 - last_point[0]);
    area /= 2;
    area.unsigned_abs() + 1 + perimeter as usize / 2
}

fn part2(input: &str) -> usize {
    let (last_point, mut area, perimeter) =
        input
            .lines()
            .fold(([0, 0], 0, 0), |([x0, y0], area, perimeter), line| {
                let hex = &line[line.find('#').unwrap() + 1..line.len() - 1];
                let dir = &hex[hex.len() - 1..];
                let steps = isize::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();

                let [x1, y1] = match dir {
                    "0" => [x0 + steps, y0],
                    "1" => [x0, y0 + steps],
                    "2" => [x0 - steps, y0],
                    "3" => [x0, y0 - steps],
                    _ => unreachable!(),
                };

                ([x1, y1], area + (y0 + y1) * (x1 - x0), perimeter + steps)
            });

    area += last_point[1] * (0 - last_point[0]);
    area /= 2;
    area.unsigned_abs() + 1 + perimeter as usize / 2
}

// thanks reddit for the shoelace formula
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 62);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 952408144115);
    }
}
