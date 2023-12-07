#[derive(Clone, Debug)]
struct Map(Vec<Range>);
impl Map {
    pub fn parse(input: &str) -> Self {
        let vec = input.lines().skip(1).map(Range::parse).collect();
        Map(vec)
    }
    pub fn convert(&self, value: u32) -> u32 {
        self.0
            .iter()
            .find(|r| r.src <= value && value < r.src + r.len)
            .map(|r| value - r.src + r.dest)
            .unwrap_or(value)
    }
}
#[derive(Debug, Copy, Clone)]
struct Range {
    pub dest: u32,
    pub src: u32,
    pub len: u32,
}
impl Range {
    pub fn new(dest: u32, src: u32, len: u32) -> Self {
        Self { dest, src, len }
    }
    pub fn parse(input: &str) -> Self {
        let n = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        Self::new(n[0], n[1], n[2])
    }
}

fn map_all_seeds(maps: &[Map], seeds: &[u32]) -> Vec<u32> {
    seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |v, m| m.convert(v)))
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.split("\n\n");
    let seeds: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let maps = lines.map(Map::parse).collect::<Vec<_>>();
    let locations1 = map_all_seeds(&maps, &seeds);
    println!("part1: {}", locations1.iter().min().unwrap());

    let ranges: Vec<_> = seeds.chunks(2).map(|w| (w[0]..w[0] + w[1])).collect();
    let locations2 = ranges
        .iter()
        .map(|range| {
            let range = range.clone();
            let maps = maps.clone();
            std::thread::spawn(move || {
                let mut min = u32::MAX;
                for r in range.into_iter() {
                    let locations = map_all_seeds(&maps, &[r]);
                    let min2 = *locations.iter().min().unwrap();
                    if min2 < min {
                        min = min2;
                    }
                }
                min
            })
        })
        .collect::<Vec<_>>();
    let locations2 = locations2
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();
    println!("part2: {}", locations2.iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn part1_example() {
        let mut lines = INPUT.split("\n\n");
        let seeds: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        let maps = lines.map(Map::parse).collect::<Vec<_>>();
        let locations = super::map_all_seeds(&maps, &seeds);
        let min = *locations.iter().min().unwrap();
        assert_eq!(min, 35);
    }
    #[test]
    fn part2_example() {
        let mut lines = INPUT.split("\n\n");
        let seeds: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .flat_map(|w| (w[0]..w[0] + w[1]))
            .collect();
        let maps = lines.map(Map::parse).collect::<Vec<_>>();
        let locations = super::map_all_seeds(&maps, &seeds);
        let min = *locations.iter().min().unwrap();
        assert_eq!(min, 46);
    }
}
