use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{self, complete::space0},
    multi, AsChar, IResult,
};

fn main() {
    let mut files = Files::parse(include_str!("../../example"))
        .unwrap()
        .1
        .files
        .into_iter()
        .collect::<Vec<_>>();
    println!("{files:?}");
    let len = files.len();
    // println!("{len}");
    let ans = files
        .iter()
        .enumerate()
        .filter(|(_, (_, a))| a.used != 0)
        .map(|(i, (_, a))| {
            let first_greater = files
                .iter()
                .position(|(_, b)| b.avail >= a.used)
                .unwrap_or(len);
            len - first_greater - if i >= first_greater { 1 } else { 0 }
        })
        .sum::<usize>();
    println!("{ans}");
}

#[derive(Debug, Clone)]
struct Files {
    files: HashMap<(usize, usize), FileData>,
    goal: (usize, usize),
}

impl Files {
    pub fn parse(data: &str) -> IResult<&str, Self> {
        let (data, _) = tag("root@ebhq-gridcenter# df -h\nFilesystem")(data)?;
        let (data, _) = space0(data)?;
        let (data, _) = tag("Size  Used  Avail  Use%\n")(data)?;
        let (i, data) = parse_lines(data)?;
        Ok((
            i,
            Self {
                goal: data
                    .iter()
                    .filter(|x| x.0 .0 == 0)
                    .max_by_key(|x| x.0 .1)
                    .unwrap()
                    .0,
                files: data
                    .into_iter()
                    .collect::<HashMap<(usize, usize), FileData>>(),
            },
        ))
    }
}

#[derive(Debug, Copy, Clone)]
struct FileData {
    size: usize,
    used: usize,
    avail: usize,
    use_percent: usize,
}

type PreHashmap = ((usize, usize), FileData);

// Parsing function
fn parse_lines(i: &str) -> IResult<&str, Vec<PreHashmap>> {
    multi::separated_list0(nom::character::complete::char('\n'), parse_line)(i)
}

fn parse_line(i: &str) -> IResult<&str, PreHashmap> {
    let (i, _) = tag("/dev/grid/node-x")(i)?;
    let (i, x) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let x = x.parse::<usize>().unwrap();
    let (i, _) = tag("-y")(i)?;
    let (i, y) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let y = y.parse::<usize>().unwrap();
    let (i, _) = space0(i)?;
    let (i, size) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let size = size.parse::<usize>().unwrap();
    let (i, _) = character::complete::char('T')(i)?;
    let (i, _) = space0(i)?;
    let (i, used) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let used = used.parse::<usize>().unwrap();
    let (i, _) = character::complete::char('T')(i)?;
    let (i, _) = space0(i)?;
    let (i, avail) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let avail = avail.parse::<usize>().unwrap();
    let (i, _) = character::complete::char('T')(i)?;
    let (i, _) = space0(i)?;
    let (i, use_percent) = nom::bytes::complete::take_while(|x: char| x.is_dec_digit())(i)?;
    let use_percent = use_percent.parse::<usize>().unwrap();
    let (i, _) = character::complete::char('%')(i)?;
    Ok((
        i,
        (
            (x, y),
            FileData {
                size,
                used,
                avail,
                use_percent,
            },
        ),
    ))
}
