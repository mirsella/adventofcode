use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    iter,
    ops::Add,
    rc::Rc,
};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Pulse {
    High,
    Low,
}
impl Add<Pulse> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, pulse: Pulse) -> Self::Output {
        match pulse {
            Pulse::High => (self.0, self.1 + 1),
            Pulse::Low => (self.0 + 1, self.1),
        }
    }
}
impl From<Pulse> for bool {
    fn from(p: Pulse) -> Self {
        match p {
            Pulse::High => true,
            Pulse::Low => false,
        }
    }
}
impl From<bool> for Pulse {
    fn from(b: bool) -> Self {
        match b {
            true => Self::High,
            false => Self::Low,
        }
    }
}
type Outputs<'a> = Vec<Rc<Box<dyn Module<'a>>>>;
trait Module<'a> {
    /// 0 low, 1 high
    fn pulse(&self, sender: &'a str, input: Pulse) -> (usize, usize);
    fn connect(&self, output: Outputs<'a>);
    fn name(&self) -> &'a str;
    fn as_any(&self) -> &(dyn Any + 'a);
}
struct Broadcaster<'a> {
    outputs: RefCell<Outputs<'a>>,
    name: &'a str,
}
impl<'a> Module<'a> for Broadcaster<'a> {
    fn pulse(&self, _: &str, input: Pulse) -> (usize, usize) {
        self.outputs
            .borrow()
            .iter()
            .map(|m| {
                println!("{} -{input:?}-> {}", self.name, m.name());
                m.pulse(self.name, input)
            })
            .fold((0, 0), |(low, high), (l, h)| (low + l, high + h) + input)
    }
    fn connect(&self, outputs: Outputs<'a>) {
        self.outputs.borrow_mut().extend(outputs)
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Broadcaster<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            outputs: RefCell::new(Vec::new()),
        }
    }
}
struct Dummy<'a> {
    name: &'a str,
}
impl<'a> Module<'a> for Dummy<'a> {
    fn pulse(&self, _: &str, _input: Pulse) -> (usize, usize) {
        (0, 0)
    }
    fn connect(&self, _outputs: Outputs<'a>) {}
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Dummy<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}
struct Flipflop<'a> {
    state: Cell<bool>,
    outputs: RefCell<Outputs<'a>>,
    name: &'a str,
}
impl<'a> Module<'a> for Flipflop<'a> {
    fn pulse(&self, _: &str, input: Pulse) -> (usize, usize) {
        if let Pulse::High = input {
            return (0, 0);
        }
        match self.state.get() {
            true => self.state.set(false),
            false => self.state.set(true),
        }
        let state = self.state.get().into();
        self.outputs
            .borrow()
            .iter()
            .map(|m| {
                println!("{} -{state:?}-> {}", self.name, m.name());
                m.pulse(self.name, state)
            })
            .fold((0, 0), |(low, high), (l, h)| (low + l, high + h) + state)
    }
    fn connect(&self, outputs: Outputs<'a>) {
        self.outputs.borrow_mut().extend(outputs)
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Flipflop<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            state: Cell::new(false),
            outputs: RefCell::new(Vec::new()),
        }
    }
}
struct Conjunction<'a> {
    name: &'a str,
    inputs: RefCell<HashMap<&'a str, Pulse>>,
    outputs: RefCell<Outputs<'a>>,
}
impl<'a> Module<'a> for Conjunction<'a> {
    fn pulse(&self, sender: &'a str, input: Pulse) -> (usize, usize) {
        let pulse = {
            let mut map = self.inputs.borrow_mut();
            map.insert(sender, input);
            map.values().all(|&p| p == Pulse::Low)
        };
        let pulse = pulse.into();
        self.outputs
            .borrow()
            .iter()
            .map(|m| {
                println!("{} -{pulse:?}-> {}", self.name, m.name());
                m.pulse(self.name, pulse)
            })
            .fold((0, 0), |(low, high), (l, h)| (low + l, high + h) + pulse)
    }
    fn connect(&self, outputs: Outputs<'a>) {
        self.outputs.borrow_mut().extend(outputs)
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Conjunction<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            inputs: RefCell::new(HashMap::new()),
            outputs: RefCell::new(Vec::new()),
        }
    }
    fn populate(&self, inputs: impl Iterator<Item = &'a str>) {
        let mut map = self.inputs.borrow_mut();
        for input in inputs {
            map.insert(input, Pulse::Low);
        }
    }
}

fn new_module<'a>(name: &'a str) -> Box<dyn Module<'a> + 'a> {
    if name == "broadcaster" {
        return Box::new(Broadcaster::new(name));
    }
    if name.starts_with('%') {
        return Box::new(Flipflop::new(name.strip_prefix('%').unwrap()));
    }
    if name.starts_with('&') {
        return Box::new(Conjunction::new(name.strip_prefix('&').unwrap()));
    }
    return Box::new(Dummy::new(name));
}

fn parse(input: &'static str) -> Vec<Rc<Box<dyn Module<'static> + 'static>>> {
    let mut conjunctions = Vec::new();

    let (modules, outputs) = input
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            let name = s.next().unwrap().trim();
            let outputs = s
                .skip(1)
                .map(|c| c.trim_matches([' ', ','].as_ref()))
                .collect::<Vec<_>>();
            let module = Rc::new(new_module(name));
            if name.starts_with('&') {
                conjunctions.push(module.clone());
            }
            (module, outputs)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    for (module, outputs) in modules.iter().zip(outputs) {
        let outputs_modules = outputs
            .iter()
            .map(|&name| {
                modules
                    .iter()
                    .find(|m| m.name() == name)
                    .cloned()
                    .unwrap_or_else(|| Rc::new(new_module(name)))
            })
            .collect::<Vec<_>>();
        module.connect(outputs_modules);
        for con in &conjunctions {
            if outputs.contains(&con.name()) {
                println!("adding {} to {} inputs", module.name(), con.name());
                con.as_any()
                    .downcast_ref::<Conjunction>()
                    .unwrap()
                    .populate(iter::once(module.name()));
            }
        }
    }
    modules
}

fn part1(input: &'static str) -> usize {
    let modules = parse(input);
    let broadcaster = modules.iter().find(|m| m.name() == "broadcaster").unwrap();
    let count = (0..4)
        .map(|_| {
            println!("button -Low-> broadcaster");
            broadcaster.pulse("button", Pulse::Low)
        })
        .fold((0, 0), |(low, high), (l, h)| (low + l + 1, high + h));
    dbg!(count);
    count.0 * count.1
}
fn part2(input: &str) -> usize {
    todo!()
}
fn main() {
    let input: &str = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 8000 * 4000);
    }
    #[test]
    fn part1_2() {
        assert_eq!(super::part1(INPUT2), 4250 * 2750);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 0);
    }
}
