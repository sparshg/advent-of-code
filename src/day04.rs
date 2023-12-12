use itertools::{enumerate, Itertools};

fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let (a, b) = line
            .split_once(':')
            .unwrap()
            .1
            .split('|')
            .map(|x| x.split_whitespace().collect_vec())
            .collect_tuple()
            .unwrap();
        let matches = b
            .iter()
            .fold(0, |matches, c| matches + if a.contains(c) { 1 } else { 0 });
        acc + if matches > 0 { 1 << matches - 1 } else { 0 }
    })
}

fn part2(input: &str) -> i32 {
    let matches = input
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once(':')
                .unwrap()
                .1
                .split('|')
                .map(|x| x.split_whitespace().collect_vec())
                .collect_tuple()
                .unwrap();
            b.iter()
                .fold(0, |matches, c| matches + if a.contains(c) { 1 } else { 0 })
        })
        .collect_vec();

    let mut res = vec![1; input.lines().count()];
    for i in 0..res.len() {
        for j in i + 1..=(i + matches.get(i).unwrap_or(&0)).min(res.len() - 1) {
            res[j] += res[i];
        }
    }
    res.iter().sum()
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
