pub fn part1(input: &str) -> usize {
    let s = input.split_once("\n\n").unwrap();
    let flows = s.0.lines().map(Flow::new).collect::<Vec<_>>();
    let parts = s.1.lines().map(Part::new).collect::<Vec<_>>();
    let mut count = 0;
    let start = flows.iter().find(|x| x.name == "in").unwrap();
    for part in parts {
        let status = start.run(&part, &flows);
        if let Action::Accepted = status {
            count += part.sum();
        }
    }
    count
}
