use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
    ops::{Add, Not},
};

use num::integer::lcm;

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
type PulseResult<'a> = (Pulse, &'a [&'a str]);
trait Module<'a> {
    fn pulse(&self, sender: &'a str, input: Pulse) -> PulseResult;
    fn name(&self) -> &'a str;
    fn as_any(&self) -> &(dyn Any + 'a);
}
struct Broadcaster<'a> {
    outputs: Vec<&'a str>,
    name: &'a str,
}
impl<'a> Module<'a> for Broadcaster<'a> {
    fn pulse(&self, _: &str, input: Pulse) -> PulseResult {
        (input, &self.outputs)
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Broadcaster<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self { name, outputs }
    }
}
struct Dummy<'a> {
    name: &'a str,
}
impl<'a> Module<'a> for Dummy<'a> {
    fn pulse(&self, _: &str, _input: Pulse) -> PulseResult {
        (Pulse::Low, &[])
    }
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
    outputs: Vec<&'a str>,
    name: &'a str,
}
impl<'a> Module<'a> for Flipflop<'a> {
    fn pulse(&self, _: &str, input: Pulse) -> PulseResult {
        if let Pulse::High = input {
            return (Pulse::Low, &[]);
        }
        match self.state.get() {
            true => self.state.set(false),
            false => self.state.set(true),
        }
        let state = self.state.get().into();
        (state, &self.outputs)
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Flipflop<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self {
            name,
            outputs,
            state: Cell::new(false),
        }
    }
}
struct Conjunction<'a> {
    name: &'a str,
    inputs: RefCell<HashMap<&'a str, Pulse>>,
    outputs: Vec<&'a str>,
}
impl<'a> Module<'a> for Conjunction<'a> {
    fn pulse(&self, sender: &'a str, input: Pulse) -> PulseResult {
        let all_high = {
            self.inputs.borrow_mut().insert(sender, input);
            self.inputs.borrow().values().all(|&p| p == Pulse::High)
        };
        let pulse = all_high.not().into();
        (pulse, &self.outputs)
    }
    fn name(&self) -> &'a str {
        self.name
    }
    fn as_any(&self) -> &(dyn Any + 'a) {
        self
    }
}
impl<'a> Conjunction<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>, inputs: Vec<&'a str>) -> Self {
        Self {
            name,
            inputs: RefCell::new(HashMap::from_iter(
                inputs.into_iter().map(|i| (i, Pulse::Low)),
            )),
            outputs,
        }
    }
}

fn new_module<'a>(
    name: &'a str,
    outputs: Vec<&'a str>,
    inputs: Vec<&'a str>,
) -> Box<dyn Module<'a> + 'a> {
    if name == "broadcaster" {
        return Box::new(Broadcaster::new(name, outputs));
    }
    if name.starts_with('%') {
        return Box::new(Flipflop::new(name.strip_prefix('%').unwrap(), outputs));
    }
    if name.starts_with('&') {
        return Box::new(Conjunction::new(
            name.strip_prefix('&').unwrap(),
            outputs,
            inputs,
        ));
    }
    return Box::new(Dummy::new(name));
}

fn parse(input: &'static str) -> Vec<Box<dyn Module<'static> + 'static>> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            let name = s.next().unwrap().trim();

            let outputs = s
                .skip(1)
                .map(|c| c.trim_matches([' ', ','].as_ref()))
                .collect();

            let inputs = input
                .lines()
                .filter_map(|line| {
                    let s = line.split_once("->").unwrap();
                    if !s.1.contains(name.trim_matches([' ', '&', '%'].as_ref())) {
                        return None;
                    }
                    Some(s.0.trim_matches([' ', '&', '%'].as_ref()))
                })
                .collect();
            new_module(name, outputs, inputs)
        })
        .collect::<Vec<_>>()
}

fn part1(input: &'static str) -> usize {
    let modules = parse(input);
    let count = (0..1000)
        .map(|_| {
            let modules = &modules;
            let mut queue: VecDeque<(&'static str, PulseResult)> = VecDeque::new();
            let mut counter = (1, 0);
            let broadcaster = modules.iter().find(|m| m.name() == "broadcaster").unwrap();
            queue.push_back((broadcaster.name(), broadcaster.pulse("button", Pulse::Low)));
            while let Some((name, (pulse, outputs))) = queue.pop_front() {
                for output in outputs {
                    counter = counter + pulse;
                    if let Some(module) = modules.iter().find(|&m| m.name() == *output) {
                        queue.push_back((module.name(), module.pulse(name, pulse)));
                    }
                }
            }
            counter
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    count.0 * count.1
}
fn part2(input: &'static str) -> usize {
    let modules = parse(input);
    let mut count = 0;
    let before_rx = input
        .lines()
        .find(|l| l.contains("rx"))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .strip_prefix(['&', '%'].as_ref())
        .unwrap();
    println!("before_rx: {before_rx}");
    let mut map = HashMap::new();
    'main: loop {
        count += 1;
        let modules = &modules;
        let mut queue: VecDeque<(&'static str, PulseResult)> = VecDeque::new();
        let broadcaster = modules.iter().find(|m| m.name() == "broadcaster").unwrap();
        queue.push_back((broadcaster.name(), broadcaster.pulse("button", Pulse::Low)));
        while let Some((name, (pulse, outputs))) = queue.pop_front() {
            for output in outputs {
                if let Some(module) = modules.iter().find(|&m| m.name() == *output) {
                    let result = (module.name(), module.pulse(name, pulse));
                    queue.push_back(result);
                    if result.1 .1 == [before_rx] && pulse == Pulse::Low {
                        if map.contains_key(module.name()) {
                            break 'main;
                        }
                        map.insert(module.name(), count);
                        println!("looped {}: {before_rx} in {count}", module.name());
                    }
                }
            }
        }
    }
    dbg!(&map);
    map.values()
        .skip(1)
        .fold(*map.values().next().unwrap(), |a, &b| {
            let ret = lcm(a, b);
            println!("lcm {a} {b} = {ret}");
            ret
        })
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
    const REALINPUT: &str = include_str!("../input.txt");
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 8000 * 4000);
    }
    #[test]
    fn part1_2() {
        assert_eq!(super::part1(INPUT2), 4250 * 2750);
    }
    #[test]
    fn part1_real() {
        assert_eq!(super::part1(REALINPUT), 743090292);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(REALINPUT), 241528184647003);
    }
}
