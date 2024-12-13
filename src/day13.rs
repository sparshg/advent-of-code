use itertools::Itertools;

fn solve(input: &str, adjust: i64) -> i64 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| {
            let s = s.split(&['+', ',', '=']).collect_vec();
            (s[1].parse::<i64>().unwrap(), s[3].parse::<i64>().unwrap())
        })
        .tuples()
        .filter_map(|(a, b, prize)| {
            let prize = (prize.0 + adjust, prize.1 + adjust);
            let n = (prize.1 * a.0 - prize.0 * a.1) / (b.1 * a.0 - b.0 * a.1);
            let m = (prize.0 - n * b.0) / a.0;
            (m * a.0 + n * b.0 == prize.0 && m * a.1 + n * b.1 == prize.1).then_some(m * 3 + n)
        })
        .sum::<i64>()
}

fn part1(input: &str) -> i32 {
    println!("{:?}", solve(input, 0));
    0
}

fn part2(input: &str) -> i32 {
    println!("{:?}", solve(input, 10000000000000));
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
