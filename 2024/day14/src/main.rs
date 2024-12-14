use itertools::Itertools;

fn part1(input: &str, wide: usize, high: usize) -> usize {
    let wide = wide as i16;
    let high = high as i16;
    input
        .lines()
        .map(|l| {
            let (mut x, mut y, vx, vy) = l
                .split(['p', '=', ',', ' ', 'v'])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i16>().unwrap())
                .collect_tuple()
                .unwrap();
            (0..100).for_each(|_| {
                x = (x + vx).rem_euclid(wide);
                y = (y + vy).rem_euclid(high);
            });
            let mut quadran = 0;
            if x > wide / 2 {
                quadran += 1;
            }
            if y > high / 2 {
                quadran += 2;
            }
            if x == wide / 2 || y == high / 2 {
                quadran += 4;
            }
            quadran
        })
        .fold([0usize; 4], |mut acc, id| {
            if id < 4 {
                acc[id] += 1;
            }
            acc
        })
        .into_iter()
        .product()
}

fn part2(input: &str, wide: usize, high: usize) -> usize {
    let wide = wide as i16;
    let high = high as i16;
    let mut robots: Vec<(i16, i16, i16, i16)> = {
        input
            .lines()
            .map(|l| {
                l.split(['p', '=', ',', ' ', 'v'])
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i16>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    };
    let mut min_safety_factor = None;
    for sec in 1..wide * high {
        let safety_factor = robots
            .iter_mut()
            .map(|(x, y, vx, vy)| {
                *x = (*x + *vx).rem_euclid(wide);
                *y = (*y + *vy).rem_euclid(high);

                let mut quadran = 0;
                if *x > wide / 2 {
                    quadran += 1;
                }
                if *y > high / 2 {
                    quadran += 2;
                }
                if *x == wide / 2 || *y == high / 2 {
                    quadran += 4;
                }
                quadran
            })
            .fold([0usize; 4], |mut acc, id| {
                if id < 4 {
                    acc[id] += 1;
                }
                acc
            })
            .into_iter()
            .product::<usize>();
        if let Some((_, mfactor)) = min_safety_factor {
            if safety_factor < mfactor {
                min_safety_factor = Some((sec, safety_factor));
            }
        } else {
            min_safety_factor = Some((sec, safety_factor));
        };
    }
    min_safety_factor.unwrap().0 as usize
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input, 101, 103));
    println!("Part 2: {}", part2(input, 101, 103));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT, 11, 7), 12);
    }
}
