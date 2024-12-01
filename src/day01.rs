use std::i32;

use itertools::Itertools;

fn input_vec(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip()
}

fn part1(input: &str) -> i32 {
    let (a, b) = input_vec(input);

    a.into_iter()
        .sorted()
        .zip(b.into_iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (a, b) = input_vec(input);
    let b = b.into_iter().counts();

    a.into_iter()
        .map(|x| x * *b.get(&x).unwrap_or(&0) as i32)
        .sum()
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
