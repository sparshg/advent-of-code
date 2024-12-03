use itertools::{sorted, Itertools};

fn input_vec(input: &str) -> [Vec<i32>; 2] {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip()
        .into()
}

fn part1(input: &str) -> i32 {
    let [a, b] = input_vec(input).map(sorted);
    a.zip(b).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(input: &str) -> i32 {
    let [a, b] = input_vec(input);
    let b = b.into_iter().counts();

    a.into_iter()
        .map(|x| x * *b.get(&x).unwrap_or(&0) as i32)
        .sum()
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
