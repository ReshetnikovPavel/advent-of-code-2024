use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), io::Error> {
    let lines = io::BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .flat_map(|split| split.parse::<_>())
                .collect::<Vec<i64>>()
        });

    let mut first = vec![];
    let mut second = vec![];

    for line in lines {
        first.push(line[0]);
        second.push(line[1]);
    }

    first.sort_unstable();
    second.sort_unstable();

    let res = first
        .iter()
        .zip(second)
        .map(|(first, second)| (first - second).abs())
        .sum::<i64>();

    println!("{}", res);
    Ok(())
}
