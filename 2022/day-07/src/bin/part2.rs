use std::cmp;
use std::collections::HashMap;

pub mod input {
    #[derive(Debug)]
    pub enum File<'a> {
        Dir(&'a str),
        File((u64, &'a str)),
    }

    #[derive(Debug)]
    pub enum Command<'a> {
        Ls,
        Cd(&'a str),
    }

    #[derive(Debug)]
    pub enum Line<'a> {
        Command(Command<'a>),
        Output(File<'a>),
    }
}

#[derive(Debug)]
struct File {
    files: HashMap<String, u64>,
    dirs: HashMap<String, File>,
}

impl File {
    fn new() -> Self {
        File {
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    fn add_file(&mut self, (name, size): (&str, u64), path: Vec<&str>) {
        self.get_path(path).files.insert(String::from(name), size);
    }

    fn add_dir(&mut self, name: &str, path: Vec<&str>) {
        self.get_path(path).dirs.insert(
            String::from(name),
            File {
                files: HashMap::new(),
                dirs: HashMap::new(),
            },
        );
    }

    fn get_path(&mut self, mut path: Vec<&str>) -> &mut Self {
        if path.is_empty() {
            self
        } else {
            let dir = path.remove(0);
            self.dirs.get_mut(dir).unwrap().get_path(path)
        }
    }

    fn size(&self) -> u64 {
        let filesize = self.files.iter().map(|(_a, b)| b).sum::<u64>();
        filesize + self.dirs.iter().map(|(_a, b)| b.size()).sum::<u64>()
    }
}

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .skip(1) // Assume that the first line is cd /
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<&str>>();
            match line[0] {
                "$" => input::Line::Command(match line[1] {
                    "ls" => input::Command::Ls,
                    "cd" => input::Command::Cd(line[2]),
                    _ => unreachable!(),
                }),
                _ => input::Line::Output(match line[0] {
                    "dir" => input::File::Dir(line[1]),
                    _ => input::File::File((line[0].parse::<u64>().unwrap(), line[1])),
                }),
            }
        })
        .collect::<Vec<input::Line>>();
    let mut files = File::new();
    let mut path = Vec::new();
    for line in input {
        match line {
            input::Line::Command(x) => match x {
                input::Command::Cd(x) => {
                    if x == ".." {
                        path.pop();
                    } else {
                        path.push(x);
                    }
                }
                input::Command::Ls => (),
            },
            input::Line::Output(x) => match x {
                input::File::Dir(name) => {
                    files.add_dir(name, path.clone());
                }
                input::File::File((size, name)) => {
                    files.add_file((name, size), path.clone());
                }
            },
        }
    }
    let target = 30000000 - (70000000 - files.size());
    println!("{}", solve(&files, target).unwrap());
}

fn solve(file: &File, target: u64) -> Option<u64> {
    let current = file.size();
    if file.size() > target {
        if let Some(x) = file.dirs
            .iter()
            .map(|(_, a)| solve(a, target))
            .filter_map(|x| x)
            .filter(|x| *x > target)
            .min() {
              Some(x)
            } else {
              Some(current)
            }
    } else {
      None
    }
}
