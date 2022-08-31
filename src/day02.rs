use itertools::Itertools;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .fold(vec![0, 0], |mut acc, curr| {
            let mut duplicates = curr.chars().sorted().dedup_with_count();
            acc[0] += duplicates.clone().any(|(x, _)| x == 2) as i32;
            acc[1] += duplicates.any(|(x, _)| x == 3) as i32;
            acc
        })
        .iter()
        .product()
}

fn part2(input: &str) -> i32 {
    for i in 0..input.lines().count() {
        let mut it = input.lines().skip(i);
        let curr = it.next().unwrap();
        for x in it {
            if curr
                .chars()
                .zip(x.chars())
                .fold(0, |acc, (x, y)| if x != y { acc + 1 } else { acc })
                == 1
            {
                println!(
                    "{}",
                    curr.chars()
                        .zip(x.chars())
                        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
                        .collect::<String>()
                );
                return 0;
            }
        }
    }
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
