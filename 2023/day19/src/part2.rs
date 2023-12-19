use crate::types::{Action, Flow};
use std::array;

fn count_accepted(flows: &[Flow], current: &Action, mut ranges: [Vec<usize>; 4]) -> usize {
    let name = match current {
        Action::Rejected => return 0,
        Action::Accepted => return ranges.iter().map(Vec::len).product(),
        Action::Flow(name) => name,
    };
    let flow = flows.iter().find(|f| &f.name == name).unwrap();
    flow.rules
        .iter()
        .map(|rule| {
            let index = "xmas".find(rule.part).unwrap();
            let mut valid = ranges.clone();
            (valid[index], ranges[index]) =
                ranges[index].iter().partition(|&&val| rule.is_valid(val));
            count_accepted(flows, &rule.action, valid)
        })
        .sum::<usize>()
        + count_accepted(flows, &flow.action, ranges)
}

pub fn part2(input: &str) -> usize {
    let s = input.split_once("\n\n").unwrap();
    let flows = s.0.lines().map(Flow::new).collect::<Vec<_>>();
    count_accepted(
        &flows,
        &Action::Flow("in".to_string()),
        array::from_fn(|_| (1..=4000).collect::<Vec<_>>()),
    )
}
