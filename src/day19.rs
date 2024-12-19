use itertools::Itertools;

fn backtrack(i: usize, towels: &[&str], pattern: &str, dp: &mut Vec<Option<u64>>) -> u64 {
    if i >= pattern.len() {
        return 1;
    }
    if let Some(res) = dp[i] {
        return res;
    }
    let mut ways = 0;
    for &towel in towels {
        if i + towel.len() <= pattern.len() && *towel == pattern[i..i + towel.len()] {
            ways += backtrack(i + towel.len(), towels, pattern, dp);
        }
    }
    dp[i] = Some(ways);
    ways
}

fn ways(input: &str) -> impl Iterator<Item = u64> + '_ {
    let towels = input.lines().next().unwrap().split(", ").collect_vec();
    input
        .lines()
        .skip(2)
        .map(move |line| backtrack(0, &towels, line, &mut vec![None; line.len()]))
}

fn part1(input: &str) -> i32 {
    ways(input).filter(|&x| x > 0).count() as i32
}

fn part2(input: &str) -> i32 {
    println!("{}", ways(input).sum::<u64>());
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
