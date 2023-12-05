fn get_calibration1(s: &str) -> u32 {
    let first = s.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = s.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    assert!(first <= 9);
    assert!(last <= 9);
    first * 10 + last
}

/// don't work for things like sevenine as Regex crate is not overlapping...
/// found out after day3 that the fancy-regex crate which support this exist....
// fn get_calibration2(s: &str) -> u32 {
//     let get_number = |s: &str| -> u32 {
//         match s {
//             "one" => 1,
//             "two" => 2,
//             "three" => 3,
//             "four" => 4,
//             "five" => 5,
//             "six" => 6,
//             "seven" => 7,
//             "eight" => 8,
//             "nine" => 9,
//             _ => s.parse().unwrap(),
//         }
//     };
//     let r = Regex::new(r"(?:[0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
//     let mut matches = r.find_iter(s);
//     let first = get_number(matches.next().map(|m| m.as_str()).unwrap_or("0"));
//     let last = get_number(
//         matches
//             .last()
//             .map(|m| m.as_str())
//             .unwrap_or(&first.to_string()),
//     );
//     assert!(first <= 9);
//     assert!(last <= 9);
//     first * 10 + last
// }

fn get_calibration2(s: impl Into<String>) -> u32 {
    let s: String = s.into();
    let s = s.replace("one", "on1e");
    let s = s.replace("two", "tw2o");
    let s = s.replace("three", "thr3e");
    let s = s.replace("four", "fo4ur");
    let s = s.replace("five", "fi5ve");
    let s = s.replace("six", "si6x");
    let s = s.replace("seven", "sev7en");
    let s = s.replace("eight", "eig8ht");
    let s = s.replace("nine", "ni9ne");
    get_calibration1(&s)
}

fn main() {
    let input1 = include_str!("../input1.txt");
    let sum1: u32 = input1.lines().map(get_calibration1).sum();
    let sum2: u32 = input1.lines().map(get_calibration2).sum();
    println!("part1: {sum1}");
    println!("part2: {sum2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = "1abc2 pqr3stu8vwx a1b2c3d4e5f treb7uchet".split_whitespace();
        let sum: u32 = input.map(get_calibration1).sum();
        assert_eq!(sum, 142);
    }
    #[test]
    fn example2() {
        let input = "two1nine eightwothree abcone2threexyz xtwone3four 4nineeightseven2 zoneight234 7pqrstsixteen msixm sevenine eighthree"
            .split_whitespace();
        let sum: u32 = input.map(get_calibration2).sum();
        assert_eq!(sum, 281 + 66 + 79 + 83);
    }
}
