use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let mut nums = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();
    for i in 0..75 {
        let mut ext = Vec::new();
        for x in nums.iter_mut() {
            if x == &0 {
                *x = 1;
                continue;
            }
            let digits = x.ilog10() + 1;
            if digits & 1 == 0 {
                let mask = 10u64.pow(digits / 2);
                ext.push(*x % mask);
                *x /= mask;
            } else {
                *x *= 2024;
            }
        }
        nums.extend(ext);
        dbg!(i, nums.iter().count());
    }
    nums.iter().count() as i32
}

fn part2(input: &str) -> i32 {
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
