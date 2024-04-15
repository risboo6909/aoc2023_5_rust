//! # aoc23_5
//!
//! This program is a humble attempt to solve the Advent of Code 2023, problem 5,
//! both parts, using a multicore brute force approach. I originally solved
//! this problem using intervals in Python, which worked instantly.
//! This Rust implementation is an experiment to understand how quickly the problem can
//! be solved using brute force in a compiled language.

mod mapper;

use anyhow;
use mapper::Mapper;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::path::Path;

fn read_input<P: AsRef<Path>>(input_path: P) -> anyhow::Result<Vec<String>> {
    let input = std::fs::read_to_string(input_path.as_ref())?;
    Ok(input
        .lines()
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(x.trim().to_string())
            }
        })
        .collect())
}

fn parse_input(input: Vec<String>) -> anyhow::Result<(Mapper, Vec<u64>)> {
    let mut mapper: Mapper = Mapper::new();
    let mut intervals = vec![];

    let seeds =
        Mapper::parse_seeds(input.first().expect("No seeds found")).expect("Error parsing seeds");

    for line in input.iter().skip(1) {
        if line.ends_with("map:") {
            if !intervals.is_empty() {
                mapper.add_map(&intervals).expect("Error parsing map");
                intervals.clear();
            }
            continue;
        }

        intervals.push(line);
    }

    mapper.add_map(&intervals).expect("Error parsing map");
    Ok((mapper, seeds))
}

fn main() {
    let (mapper, seeds) = read_input("./input")
        .and_then(parse_input)
        .expect("Error parsing input file");

    println!(
        "part 1: {:?}",
        seeds.iter().map(|seed| mapper.project(*seed)).min()
    );

    println!(
        "part 2: {:?}",
        seeds
            .par_iter()
            .enumerate()
            .step_by(2)
            .map(|(i, start)| {
                (*start..*start + seeds[i + 1])
                    .map(|seed| mapper.project(seed))
                    .min()
                    .unwrap()
            })
            .min()
    );
}
