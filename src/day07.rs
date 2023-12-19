use itertools::Itertools;

fn process_input(input: &str) -> impl Iterator<Item = (&'_ str, i32)> {
    input.lines().map(|x| {
        x.split_once(' ')
            .map(|(x, y)| (x, y.parse::<i32>().unwrap()))
            .unwrap()
    })
}

fn to_base_15(card: &str, part1: bool) -> u32 {
    card.chars()
        .map(|x| {
            x.to_digit(10).unwrap_or_else(|| match x {
                'T' => 10,
                'J' => {
                    if part1 {
                        11
                    } else {
                        1
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("{} not allowed", x),
            })
        })
        .fold(0, |acc, x| acc * 15 + x)
}

fn part1(input: &str) -> i32 {
    process_input(input)
        .sorted_unstable_by_key(|(a, _)| {
            let charmap = a.chars().counts();
            // 51' 42' 32' 33' 23' 24' 15' where x' = 5 - x
            let rank = charmap.values().max().unwrap() * 10 + (5 - charmap.len());
            (rank, to_base_15(a, true))
        })
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as i32 + 1) * bid)
}

fn part2(input: &str) -> i32 {
    process_input(input)
        .sorted_unstable_by_key(|(a, _)| {
            let mut charmap = a.chars().counts();
            if charmap.len() > 1 {
                if let Some(jokers) = charmap.remove(&'J') {
                    *charmap.values_mut().max().unwrap() += jokers;
                }
            }
            let rank = charmap.values().max().unwrap() * 10 + (5 - charmap.len());
            (rank, to_base_15(a, false))
        })
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as i32 + 1) * bid)
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
