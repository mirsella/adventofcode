fn count_accepted(flows: &[Flow], current: &str, mut ranges: [Vec<usize>; 4]) -> usize {
    match current {
        "R" => return 0,
        "A" => return ranges.iter().map(|v| v.len()).product(),
        _ => (),
    };
    let mut count = 0;
    let flow = flows.iter().find(|x| x.name == current).unwrap();
    for rule in &flow.rules {
        let i = "xmas".chars().position(|c| c == rule.part).unwrap();
        let mut filtered = ranges.clone();
        (filtered[i], ranges[i]) = ranges[i].iter().partition(|&&val| {
            if rule.op == '<' {
                val < rule.value
            } else {
                val > rule.value
            }
        });
        count += count_accepted(flows, rule.action.as_ref(), filtered);
    }
    count += count_accepted(flows, flow.action.as_ref(), ranges);
    count
}
pub fn part2(input: &str) -> usize {
    let s = input.split_once("\n\n").unwrap();
    let flows = s.0.lines().map(Flow::new).collect::<Vec<_>>();
    count_accepted(
        &flows,
        "in",
        array::from_fn(|_| (1..=4000).collect::<Vec<_>>()),
    )
}
