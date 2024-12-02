use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), io::Error> {
    let reports = io::BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .flat_map(|split| split.parse::<_>())
                .collect::<Vec<i64>>()
        });

    let res = reports
        .filter(|r| {
            r.windows(2).all(|p| (0..=3).contains(&(p[0] - p[1]).abs()))
                && (r.windows(2).all(|p| p[0] < p[1])
                || r.windows(2).all(|p| p[0] > p[1]))
        })
        .count();

    println!("{}", res);
    Ok(())
}
