use regex::Regex;

/// the colors are the max value
#[derive(Default, Debug)]
struct Game {
    pub id: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    /// input is a single line: Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    pub fn parse(input: &str) -> Self {
        let r = Regex::new(r"(Game [0-9]+|[0-9]+ (green|blue|red))").unwrap();
        let mut matches = r.find_iter(input).map(|m| m.as_str()).peekable();
        let first = matches.peek_mut().unwrap();
        let numbergame = first
            .split_whitespace()
            .rev()
            .fold(String::new(), |a, b| a + b + " ");
        *first = &numbergame;
        let mut game = Game::default();
        for m in matches {
            let mut s = m.split_whitespace();
            let n = s.next().unwrap().parse::<u32>().unwrap();
            // do whatever we want with n, here take the max
            match s.next().unwrap() {
                "green" => game.green = n.max(game.green),
                "blue" => game.blue = n.max(game.blue),
                "red" => game.red = n.max(game.red),
                "Game" => game.id = n,
                text => panic!("unexpected type {text}"),
            }
        }
        game
    }
    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        if self.red > red || self.green > green || self.blue > blue {
            return false;
        }
        true
    }
}
fn part1(input: &str) -> u32 {
    let mut possible_games = Vec::new();
    for line in input.lines() {
        let game = Game::parse(line);
        if game.is_possible(12, 13, 14) {
            possible_games.push(game);
        }
    }
    let sum: u32 = possible_games.iter().map(|g| g.id).sum();
    sum
}
fn part2(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .map(Game::parse)
        .map(|g| g.red * g.green * g.blue)
        .sum();
    sum
}

fn main() {
    let input = include_str!("../input.txt");
    println!("sum of possible games: {}", part1(input));
    println!("power of the games: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn game() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::parse(line);
        assert_eq!(game.id, 3);
        assert_eq!(game.red, 20);
        assert_eq!(game.green, 13);
        assert_eq!(game.blue, 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(part1(INPUT), 8);
    }
    #[test]
    fn example_part2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
