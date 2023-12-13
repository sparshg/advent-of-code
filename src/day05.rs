use itertools::Itertools;

fn get_seeds_and_maps(input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();
    let maps = input
        .split(":\n")
        .flat_map(|x| x.split("\n\n"))
        .filter(|x| x.chars().next().unwrap().is_numeric())
        .map(|x| {
            x.lines()
                .map(|y| {
                    y.split_whitespace()
                        .map(|z| z.parse::<i64>().unwrap())
                        .collect_tuple::<(i64, i64, i64)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();
    (seeds, maps)
}

fn part1(input: &str) -> i32 {
    let (mut seeds, maps) = get_seeds_and_maps(input);
    for s in &mut seeds {
        for m in &maps {
            if let Some(v) = m.iter().find(|m| (m.1..m.1 + m.2).contains(s)) {
                *s = *s + v.0 - v.1;
            }
        }
    }
    *seeds.iter().min().unwrap() as i32
}

fn part2(input: &str) -> i32 {
    let (mut seeds, maps) = get_seeds_and_maps(input);
    // maps[0].iter().find(|m| (m.1..m.1 + m.2).contains(seeds[0]))
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
