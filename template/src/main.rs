fn part1() -> usize {
    todo!()
}
fn part2() -> usize {
    todo!()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "{{example}}";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), {{example_eq}});
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), {{example_eq}});
    }
}
