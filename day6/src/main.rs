struct Race {
    pub time: usize,
    pub distance: usize,
}
impl Race {
    pub fn press_button(&self, time: usize) -> usize {
        let speed = time;
        let remaining = self.time - time;
        speed * remaining
    }
    pub fn test_all(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.time).map(|x| self.press_button(x))
    }
}
struct Races(Vec<Race>);
impl Races {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let time = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap());
        let distance = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap());
        let races = time
            .zip(distance)
            .map(|(time, distance)| Race { time, distance })
            .collect();
        Self(races)
    }
}
fn part1(input: &str) -> usize {
    let races = Races::parse(input);
    let all = races
        .0
        .iter()
        .map(|x| x.test_all().filter(|&d| d > x.distance).count())
        .collect::<Vec<_>>();
    all.iter().product::<usize>()
}
fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();
    let race = Race { time, distance };
    race.test_all().filter(|&d| d > distance).count()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn part1_example() {
        assert_eq!(super::part1(INPUT), 288);
    }
    #[test]
    fn part2_example() {
        assert_eq!(super::part2(INPUT), 71503);
    }
}
