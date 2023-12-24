use itertools::Itertools;
use std::ops::RangeBounds;
use z3::ast::{Ast, Int};

fn part1(input: &str, range: impl RangeBounds<f64>) -> usize {
    let hailstones = input.lines().map(|l| {
        let mut s = l
            .split([',', '@'])
            .map(|s| s.trim().parse::<f64>().unwrap());
        (
            s.by_ref().take(3).collect_tuple::<(_, _, _)>().unwrap(),
            s.collect_tuple::<(_, _, _)>().unwrap(),
        )
    });
    hailstones
        .tuple_combinations()
        .filter(
            |(((x1, y1, _), (dx1, dy1, _)), ((x2, y2, _), (dx2, dy2, _)))| {
                let m1 = dy1 / dx1;
                let m2 = dy2 / dx2;
                if m1 == m2 {
                    return false;
                }
                let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
                let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);

                if dx1.signum() != (x - x1).signum() || dx2.signum() != (x - x2).signum() {
                    return false;
                }
                range.contains(&x) && range.contains(&y)
            },
        )
        .count()
}

fn part2(input: &str) -> usize {
    let hailstones = input.lines().map(|l| {
        let mut s = l
            .split([',', '@'])
            .map(|s| s.trim().parse::<f64>().unwrap());
        (
            s.by_ref().take(3).collect_tuple::<(_, _, _)>().unwrap(),
            s.collect_tuple::<(_, _, _)>().unwrap(),
        )
    });

    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

    for (i, ((x, y, z), (dx, dy, dz))) in hailstones.enumerate() {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _));
        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    s.check();
    let res = s
        .get_model()
        .unwrap()
        .eval(&(&fx + &fy + &fz), false)
        .unwrap();
    res.as_u64().unwrap() as usize
}

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "Part 1: {}",
        part1(input, 200000000000000f64..=400000000000000f64)
    );
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT, 7f64..=27f64), 2);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 47);
    }
}
