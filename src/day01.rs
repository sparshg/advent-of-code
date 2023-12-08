use std::{collections::HashMap, i32};

fn part1(input: &str) -> i32 {
    input.lines().fold(0, |sum, line| {
        sum + line
            .chars()
            .find(|&x| x.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
            * 10
            + line
                .chars()
                .rev()
                .find(|&x| x.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap() as i32
    })
}

fn part2(input: &str) -> i32 {
    let m = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);
    input.lines().fold(0, |sum, line: &str| {
        let (mut first, mut last) = (None, None);
        for (i, c) in line.chars().enumerate() {
            if let Some(num) = m
                .get(&line.get(i..i + 3).unwrap_or_default())
                .or(m.get(&line.get(i..i + 4).unwrap_or_default()))
                .or(m.get(&line.get(i..i + 5).unwrap_or_default()))
                .map(|&x| x)
                .or(c.to_digit(10))
            {
                if first.is_none() {
                    first = Some(num);
                }
                last = Some(num);
            }
        }
        sum + (first.unwrap() * 10 + last.unwrap()) as i32
    })
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
