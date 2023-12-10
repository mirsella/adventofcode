use super::part1::get_tunnel_parts;

pub fn part2(input: &str) -> usize {
    let map: Vec<char> = input.chars().collect();
    let line = map.iter().position(|&c| c == '\n').unwrap() + 1;
    let parts = get_tunnel_parts(input);
    let mut count = 0usize;
    let mut inside = false;
    for p in 0..map.len() {
        if parts.contains(&p) {
            match map[p] {
                '|' | 'L' | 'J' => inside = !inside,
                'S' if p > line
                    && (map[p - line] == '|' || map[p - line] == 'F' || map[p - line] == '7') =>
                {
                    inside = !inside
                }
                _ => (),
            }
        } else {
            count += inside as usize;
        }
        if p % line == line - 1 {
            inside = false;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const INPUT2: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    const INPUT3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const INPUT4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    #[test]
    fn ex1() {
        assert_eq!(super::part2(INPUT), 4);
    }
    #[test]
    fn ex2() {
        assert_eq!(super::part2(INPUT2), 4);
    }
    #[test]
    fn ex3() {
        assert_eq!(super::part2(INPUT3), 8);
    }
    #[test]
    fn ex4() {
        assert_eq!(super::part2(INPUT4), 10);
    }
}
