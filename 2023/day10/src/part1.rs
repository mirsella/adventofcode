use std::collections::HashSet;

fn is_connected(input: &[u8], p: usize, n: usize) -> bool {
    let line = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    if p + 1 == n {
        matches!(input[p] as char, 'S' | '-' | 'L' | 'F')
            && matches!(input[n] as char, 'S' | '-' | 'J' | '7')
    } else if p.checked_sub(1) == Some(n) {
        matches!(input[p] as char, 'S' | '-' | 'J' | '7')
            && matches!(input[n] as char, 'S' | '-' | 'L' | 'F')
    } else if p + line == n {
        matches!(input[p] as char, 'S' | '|' | 'F' | '7')
            && matches!(input[n] as char, 'S' | '|' | 'L' | 'J')
    } else if p.checked_sub(line) == Some(n) {
        matches!(input[p] as char, 'S' | '|' | 'L' | 'J')
            && matches!(input[n] as char, 'S' | '|' | 'F' | '7')
    } else {
        false
    }
}
fn find_next(input: &[u8], position: usize, previous: usize) -> usize {
    let line = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    if position + 1 != previous && is_connected(input, position, position + 1) {
        position + 1
    } else if position.checked_sub(1) != Some(previous)
        && is_connected(input, position, position - 1)
    {
        position - 1
    } else if position + line != previous && is_connected(input, position, position + line) {
        position + line
    } else if position.checked_sub(line) != Some(previous)
        && is_connected(input, position, position - line)
    {
        position - line
    } else {
        panic!("No valid path");
    }
}
pub fn part1(input: &str) -> usize {
    let mut count = 0;
    let s = input.find('S').unwrap();
    let mut pos = s;
    let mut previous = pos;
    loop {
        let next = find_next(input.as_bytes(), pos, previous);
        count += 1;
        previous = pos;
        pos = next;
        if next == s {
            break;
        }
    }
    dbg!(count);
    count / 2
}

// for part2
pub fn get_tunnel_parts(input: &str) -> HashSet<usize> {
    let s = input.find('S').unwrap();
    let mut pos = s;
    let mut previous = pos;
    let mut set = HashSet::new();
    loop {
        let next = find_next(input.as_bytes(), pos, previous);
        set.insert(pos);
        previous = pos;
        pos = next;
        if next == s {
            break;
        }
    }
    set
}

#[cfg(test)]
mod tests {
    const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    #[test]
    fn ex1() {
        assert_eq!(super::part1(INPUT), 4);
    }
    #[test]
    fn ex2() {
        assert_eq!(super::part1(INPUT2), 8);
    }
}
