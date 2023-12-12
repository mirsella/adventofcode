use std::iter;

fn find_all_permutations(pattern: &str, counts: impl Iterator<Item = usize>) -> usize {
    let mut bytes = vec![b'.'];
    bytes.extend(pattern.trim_end_matches('.').as_bytes());
    let pattern = bytes;
    let mut vec = vec![0; pattern.len() + 1];
    vec[0] = 1;

    for (i, _) in pattern.iter().take_while(|&&c| c != b'#').enumerate() {
        vec[i + 1] = 1;
    }

    for count in counts {
        let mut nvec = vec![0; pattern.len() + 1];
        let mut chunk = 0;

        for (i, &c) in pattern.iter().enumerate() {
            match c == b'.' {
                true => chunk = 0,
                false => chunk += 1,
            }
            if c != b'#' {
                nvec[i + 1] += nvec[i];
            }
            if chunk >= count && pattern[i - count] != b'#' {
                nvec[i + 1] += vec[i - count];
            }
        }
        vec = nvec;
    }
    vec.last().copied().unwrap()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (pattern, count) = l.split_once(' ').unwrap();
            let counts = count.split(',').map(|c| c.parse().unwrap());
            find_all_permutations(pattern, counts)
        })
        .sum()
}
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (pattern, count) = l.split_once(' ').unwrap();
            let counts: Vec<usize> = count.split(',').map(|c| c.parse().unwrap()).collect();
            let unfolded_pattern = (0..4)
                .map(|_| pattern.to_string() + "?")
                .chain(iter::once(pattern.to_string()))
                .collect::<String>();
            let unfolded_counts = counts.iter().copied().cycle().take(5 * counts.len());
            find_all_permutations(&unfolded_pattern, unfolded_counts)
        })
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 21);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 525152);
    }
}
