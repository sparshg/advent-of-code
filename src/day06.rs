use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let input = input
        .lines()
        .take(2)
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|y| y.parse::<f64>().unwrap())
                .collect_vec()
        })
        .collect_tuple::<(Vec<f64>, Vec<f64>)>()
        .unwrap();
    input
        .0
        .into_iter()
        .zip(input.1)
        .map(|(n, k)| {
            (((n + (n * n - 4. * k).sqrt()) * 0.5 - 1.).ceil()
                - ((n - (n * n - 4. * k).sqrt()) * 0.5 + 1.).floor()) as i32
                + 1
        })
        .product()
}

fn part2(input: &str) -> i32 {
    // modify input manually, run part 1
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    return Some(part1(input));
    // return Some(part2(input));
    None
}
