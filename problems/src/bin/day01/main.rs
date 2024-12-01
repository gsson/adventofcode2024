use std::collections::HashMap;

use adventofcode2024_common::StrExt as _;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let (mut firsts, mut seconds) = input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(first, second)| (first.to_u32(), second.to_u32()))
        .fold(
            (Vec::new(), Vec::new()),
            |(mut firsts, mut seconds), (f, s)| {
                firsts.push(f);
                seconds.push(s);
                (firsts, seconds)
            },
        );

    firsts.sort();
    seconds.sort();

    firsts
        .into_iter()
        .zip(seconds)
        .map(|(f, s)| f.abs_diff(s))
        .sum()
}

fn part2(input: &str) -> u32 {
    let (firsts, seconds) = input.lines().flat_map(|line| line.split_once("   ")).fold(
        (Vec::new(), HashMap::<&str, u32>::new()),
        |(mut firsts, mut seconds), (f, s)| {
            firsts.push(f);
            *seconds.entry(s).or_default() += 1;
            (firsts, seconds)
        },
    );

    firsts
        .into_iter()
        .map(|f| f.to_u32() * seconds.get(f).unwrap_or(&0))
        .sum()
}

#[test]
fn part1_example() {
    assert_eq!(11, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(2176849, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(31, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(23384288, part2(INPUT));
}
