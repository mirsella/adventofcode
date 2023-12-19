#[derive(Clone)]
pub struct Rule {
    pub part: char,
    pub op: char,
    pub value: usize,
    pub action: Action,
}
impl Rule {
    pub fn new(input: &str) -> Self {
        let part = input.chars().nth(0).unwrap();
        let op = input.chars().nth(1).unwrap();
        let value: usize = input[2..input.find(':').unwrap()].parse().unwrap();
        let action = Action::new(&input[input.find(':').unwrap() + 1..]);
        Self {
            part,
            op,
            value,
            action,
        }
    }
    pub fn is_valid(&self, part: usize) -> bool {
        match self.op {
            '<' => part < self.value,
            '>' => part > self.value,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Clone)]
pub enum Action {
    Flow(String),
    Rejected,
    Accepted,
}
impl Action {
    pub fn new(input: &str) -> Self {
        match input {
            "R" => Self::Rejected,
            "A" => Self::Accepted,
            _ => Self::Flow(input.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct Flow {
    pub name: String,
    pub rules: Vec<Rule>,
    pub action: Action,
}
impl Flow {
    pub fn new(input: &str) -> Self {
        let mut s = input.split(['{', '}']);
        let name = s.next().unwrap().to_string();
        let mut flows = s.next().unwrap().split(',').collect::<Vec<_>>();
        let action = Action::new(flows.pop().unwrap());
        let flows = flows.iter().map(|x| Rule::new(x)).collect();
        Flow {
            name,
            rules: flows,
            action,
        }
    }
    pub fn run(&self, parts: &Part, flows: &[Flow]) -> Action {
        for rule in &self.rules {
            let value = parts.get(rule.part);
            let status = match rule.op {
                '<' => value < rule.value,
                '>' => value > rule.value,
                _ => panic!("Invalid operator"),
            };
            if !status {
                continue;
            }
            match &rule.action {
                Action::Flow(name) => {
                    let flow = flows.iter().find(|&x| x.name == *name).unwrap();
                    return flow.run(parts, flows);
                }
                s => return s.clone(),
            }
        }
        match &self.action {
            Action::Flow(name) => {
                let flow = flows.iter().find(|&x| x.name == *name).unwrap();
                flow.run(parts, flows)
            }
            s => s.clone(),
        }
    }
}

pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}
impl Part {
    pub fn get(&self, part: char) -> usize {
        match part {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid part"),
        }
    }
    pub fn new(input: &str) -> Self {
        let mut parts = input[1..input.len() - 1].split(',');
        let x = parts.next().unwrap()[2..].parse().unwrap();
        let m = parts.next().unwrap()[2..].parse().unwrap();
        let a = parts.next().unwrap()[2..].parse().unwrap();
        let s = parts.next().unwrap()[2..].parse().unwrap();
        Self { x, m, a, s }
    }
    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
