use std::ops::Range;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", count_parts(input));
    println!("Part 2: {}", count_gear_ratio(input));
}

fn count_parts(s: &str) -> usize {
    let mut sum = 0;
    let line_length = s.find('\n').unwrap();
    let s = s.replace('\n', "");
    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(&s) {
        let range = m.start().saturating_sub(1)..m.end().saturating_add(1);
        let aboverange = range.start.wrapping_sub(line_length)..range.end.wrapping_sub(line_length);
        let underrange =
            range.start.saturating_add(line_length)..range.end.saturating_add(line_length);
        let find_symbol = |r: Range<usize>| {
            s.get(r)
                .is_some_and(|s| s.chars().any(|c| c.is_ascii_punctuation() && c != '.'))
        };
        if find_symbol(range) || find_symbol(aboverange) || find_symbol(underrange) {
            sum += m.as_str().parse::<usize>().unwrap();
        }
    }
    sum
}
fn count_gear_ratio(s: &str) -> usize {
    let mut sum = 0;
    let line_length = s.find('\n').unwrap();
    let s = s.replace('\n', "");
    let redigit = Regex::new(r"\d+").unwrap();
    let matches = s.match_indices('*');
    let digits = redigit.find_iter(&s).collect::<Vec<_>>();
    for m in matches {
        let (i, _) = m;
        let adjacent_index = [
            i - 1,
            i,
            i + 1,
            i - line_length - 1,
            i - line_length,
            i - line_length + 1,
            i + line_length - 1,
            i + line_length,
            i + line_length + 1,
        ];
        let mut nums = vec![];
        for digit in &digits {
            let validdigit = adjacent_index
                .into_iter()
                .any(|i| digit.start() == i || digit.end() - 1 == i);
            if validdigit {
                nums.push(digit.as_str().parse::<usize>().unwrap());
            }
        }
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn count_parts_example() {
        assert_eq!(super::count_parts(INPUT), 4361);
    }

    #[test]
    fn count_gear_ratio_example() {
        assert_eq!(super::count_gear_ratio(INPUT), 467835);
    }
}
