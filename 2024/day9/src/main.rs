fn part1(input: &str) -> usize {
    let mut disk = input
        .chars()
        .fold((Vec::new(), true, 0), |(mut acc, is_file, mut id), char| {
            let n = char.to_digit(10).unwrap();
            if is_file {
                (0..n).for_each(|_| acc.push(Some(id)));
            } else {
                (0..n).for_each(|_| acc.push(None));
                id += 1;
            }
            (acc, !is_file, id)
        })
        .0;
    let mut count = 0;
    for i in 0..disk.len() {
        if disk[i].is_none() {
            if let Some(last_rev) = disk.iter().skip(i).rev().position(|&c| c.is_some()) {
                let last = disk.len() - last_rev - 1;
                disk[i] = disk[last];
                disk[last] = None;
            }
        }
        if let Some(id) = disk[i] {
            count += id * i;
        }
    }
    count
}
fn part2(input: &str) -> usize {
    let mut disk = input
        .chars()
        .fold((Vec::new(), true, 0), |(mut acc, is_file, mut id), char| {
            let n = char.to_digit(10).unwrap();
            if is_file {
                (0..n).for_each(|_| acc.push(Some(id)));
            } else {
                (0..n).for_each(|_| acc.push(None));
                id += 1;
            }
            (acc, !is_file, id)
        })
        .0;
    let mut i = disk.len() - 1;
    loop {
        let mut file_size = 0;
        if let Some(id) = disk[i] {
            while i - file_size > 0 && disk[i - file_size].is_some_and(|c| c == id) {
                file_size += 1;
            }
        }
        if file_size > 0 {
            for j in 0..i {
                let mut empty_size = 0;
                while j + empty_size < i && disk[j + empty_size].is_none() {
                    empty_size += 1;
                }
                if empty_size >= file_size {
                    (0..file_size).for_each(|k| {
                        disk[j + k] = disk[i - k];
                        disk[i - k] = None;
                    });
                }
            }
            i -= file_size;
        } else if i > 1 {
            i -= 1;
        } else {
            break;
        }
    }
    disk.iter().enumerate().fold(0, |mut acc, (i, c)| {
        if let Some(id) = c {
            acc += id * i;
        }
        acc
    })
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2333133121414131402";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1928);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2858);
    }
}
