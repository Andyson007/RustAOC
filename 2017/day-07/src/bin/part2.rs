use std::collections::{HashMap, HashSet};
#[derive(Debug, Eq, PartialEq, Clone)]
struct Program {
    name: String,
    programs: Vec<Program>,
    size: usize,
}

impl Program {
    fn size(&self) -> usize {
        self.size + self.programs.iter().map(|p| p.size()).sum::<usize>()
    }

    fn insert(&mut self, loc: &str, what: (&str, usize)) -> bool {
        if self.name == loc {
            self.programs.push(Program {
                name: what.0.to_string(),
                size: what.1,
                programs: Vec::new(),
            });
            true
        } else {
            for program in &mut self.programs {
                if program.insert(loc, what) {
                    return true;
                }
            }
            false
        }
    }

    fn contains(&self, name: &str) -> bool {
        if self.name == name.to_string() {
            true
        } else {
            self.programs.iter().any(|x| x.contains(name))
        }
    }
}

fn main() {
    let mut programs: HashMap<&str, (usize, HashSet<&str>)> = HashMap::new();
    for line in include_str!("../../input.txt").lines() {
        let split = line.split(" -> ").collect::<Vec<&str>>();
        let first = split[0].split(" (").collect::<Vec<&str>>();
        let size = first[1]
            .chars()
            .take_while(|x| *x != ')')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        if let Some(x) = split.iter().nth(1) {
            let subprograms = x.split(", ").collect::<HashSet<&str>>();
            programs.insert(first[0], (size, subprograms));
        } else {
            programs.insert(first[0], (size, HashSet::new()));
        }
    }

    let mut mapped_to: HashSet<&str> = HashSet::new();
    for (_k, program) in &programs {
        for dest in &program.1 {
            mapped_to.insert(dest);
        }
    }

    let mut base = None;
    let mut to_eval = programs.clone();
    for (k, program) in &programs {
        if !mapped_to.contains(k) {
            base = Some(Program {
                name: k.to_string(),
                size: program.0,
                programs: Vec::new(),
            });
        }
    }
    let mut base = base.unwrap();
    while to_eval.len() > 0 {
        for (k, v) in to_eval.clone() {
            if base.contains(k) {
                for topush in v.1 {
                    base.insert(k, (topush, programs.get(&topush).unwrap().0));
                }
                to_eval.remove(k);
            }
        }
    }
    let mut other = 0;
    loop {
        let sizes = base
            .programs
            .iter()
            .map(|p| p.size())
            .collect::<Vec<usize>>();
      println!("{sizes:?}");
        let search = sizes.iter().fold(0, |sum, curr| sum ^ curr);
        if !base
            .programs
            .windows(2)
            .all(|arr| arr[0].size() == arr[1].size())
        {
            if let Some(x) = base.programs.iter().find(|x| x.size() == search) {
                other = *sizes.iter().find(|x| **x != search).unwrap();
                base = x.clone();
            }
        } else {
          let size = base.size();
            println!("{}", other);
            println!("{size}");
            if size> other {
              println!("{}", base.size-size.abs_diff(other));
            } else {
              println!("{}", base.size+size.abs_diff(other));
            }
            break;
        }
    }
}
