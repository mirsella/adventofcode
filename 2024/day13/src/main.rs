use regex::Regex;

#[derive(Debug)]
struct Machine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}

impl Machine {
    fn from_list(input: &str) -> Vec<Machine> {
        let regex = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        regex
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [ax, ay, bx, by, px, py])| Machine {
                a: (ax.parse().unwrap(), ay.parse().unwrap()),
                b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (px.parse().unwrap(), py.parse().unwrap()),
            })
            .collect()
    }
}

fn solve_integer(ax: isize, ay: isize, bx: isize, by: isize, tx: isize, ty: isize) -> usize {
    // (thanks google)
    // calculate b using the derived formula:
    // given the system:
    //   a*ax + b*ay = tx
    //   a*bx + b*by = ty
    // after elimination, we get:
    //   b = (ty*ax - tx*ay) / (by*ax - bx*ay)
    let b = (ty * ax - tx * ay) / (by * ax - bx * ay);

    // using b, solve for a:
    // from a*ax + b*bx = tx -> a = (tx - b*bx) / ax
    let a = (tx - b * bx) / ax;

    // verify
    if (ax * a + bx * b, ay * a + by * b) != (tx, ty) {
        return 0;
    }
    a as usize * 3 + b as usize
}

// not working 🤷
fn _solve_with_good_lp(ax: f64, ay: f64, bx: f64, by: f64, tx: f64, ty: f64) -> Option<u64> {
    use good_lp::*;
    variables! {
        vars:
            a (integer) >= 0;
            b (integer) >= 0;
    }
    let model = vars
        .minimise(a + b)
        .using(default_solver)
        .with(constraint!(a * ax + b * bx == tx))
        .with(constraint!(a * ay + b * by == ty));

    let solution = match model.solve() {
        Ok(sol) => sol,
        Err(_) => return None,
    };

    let a_val = solution.value(a);
    let b_val = solution.value(b);

    Some(a_val.round() as u64 * 3 + b_val.round() as u64)
}

fn part1(input: &str) -> usize {
    let machines = Machine::from_list(input);
    let mut used = 0;
    for Machine { a, b, prize } in machines {
        used += solve_integer(
            a.0 as isize,
            a.1 as isize,
            b.0 as isize,
            b.1 as isize,
            prize.0 as isize,
            prize.1 as isize,
        );
    }
    used
}

fn part2(input: &str) -> usize {
    let machines = Machine::from_list(input);
    let mut used = 0;
    for Machine { a, b, prize } in machines {
        used += solve_integer(
            a.0 as isize,
            a.1 as isize,
            b.0 as isize,
            b.1 as isize,
            prize.0 as isize + 10000000000000,
            prize.1 as isize + 10000000000000,
        );
    }
    used
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 480);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 100);
    }
}
