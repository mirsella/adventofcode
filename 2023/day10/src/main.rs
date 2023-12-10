mod part1;
mod part2;
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1::part1(input));
    println!("Part 2: {}", part2::part2(input));
}
