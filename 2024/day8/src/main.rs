use glam::Vec2;
use pathfinding::matrix::Matrix;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut set = HashSet::new();
    for (apos, c) in m.items() {
        if *c == '.' {
            continue;
        }
        for (bpos, cb) in m.items() {
            if c != cb || apos == bpos {
                continue;
            }
            let avec = Vec2::new(apos.1 as f32, apos.0 as f32);
            let bvec = Vec2::new(bpos.1 as f32, bpos.0 as f32);
            let anewvec = avec.lerp(bvec, 2.);
            let bnewvec = bvec.lerp(avec, 2.);
            let anew = (anewvec.y as usize, anewvec.x as usize);
            let bnew = (bnewvec.y as usize, bnewvec.x as usize);
            if anewvec.x >= 0. && anewvec.y >= 0. && m.within_bounds(anew) {
                set.insert(anew);
            }
            if bnewvec.x >= 0. && bnewvec.y >= 0. && m.within_bounds(bnew) {
                set.insert(bnew);
            }
        }
    }
    set.len()
}
fn part2(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut set = HashSet::new();
    for (apos, c) in m.items() {
        if *c == '.' {
            continue;
        }
        for (bpos, cb) in m.items().skip(apos.0 + apos.1) {
            if c != cb || apos == bpos {
                continue;
            }
            let avec = Vec2::new(apos.1 as f32, apos.0 as f32);
            let bvec = Vec2::new(bpos.1 as f32, bpos.0 as f32);
            let mut factor = 1.;
            loop {
                let mut out = [false; 2];
                let anewvec = avec + (bvec - avec) * factor;
                let bnewvec = bvec + (avec - bvec) * factor;
                let anew = (anewvec.y as usize, anewvec.x as usize);
                let bnew = (bnewvec.y as usize, bnewvec.x as usize);
                if anewvec.x >= 0. && anewvec.y >= 0. && m.within_bounds(anew) {
                    set.insert(anew);
                } else {
                    out[0] = true;
                }
                if bnewvec.x >= 0. && bnewvec.y >= 0. && m.within_bounds(bnew) {
                    set.insert(bnew);
                } else {
                    out[1] = true;
                }
                if out[0] && out[1] {
                    break;
                }
                factor += 1.;
            }
        }
    }
    set.len()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 14);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 34);
    }
}
