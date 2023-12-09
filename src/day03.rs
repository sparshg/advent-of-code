use itertools::Itertools;

fn part1(input: &str) -> i32 {
    input.lines().enumerate().fold(0, |sum, (i, line)| {
        let mut start = None;
        let mut part = 0;
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if start.is_none() {
                    start = Some(j);
                }
                if match line.chars().nth(j + 1) {
                    Some(x) => !x.is_numeric(),
                    None => true,
                } {
                    let found = [-1i32, 1].into_iter().any(|x| {
                        input
                            .lines()
                            .nth((i as i32 + x) as usize)
                            .and_then(|l| {
                                l.get(start.unwrap()..j + 1).and_then(|l| {
                                    l.chars().any(|x| !x.is_numeric() && x != '.').then_some(0)
                                })
                            })
                            .is_some()
                    }) || {
                        [-1, 0i32, 1]
                            .into_iter()
                            .cartesian_product([start.unwrap() as i32 - 1i32, j as i32 + 1])
                            .any(|(x, y)| {
                                input
                                    .lines()
                                    .nth((i as i32 + x) as usize)
                                    .and_then(|l| {
                                        l.chars().nth(y as usize).and_then(|c| {
                                            (!c.is_numeric() && c != '.').then_some(0)
                                        })
                                    })
                                    .is_some()
                            })
                    };
                    if found {
                        part += line
                            .get(start.unwrap()..j + 1)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                    }
                    start = None;
                }
            }
        }
        sum + part
    })
}

fn part2(input: &str) -> i32 {
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    return Some(part1(input));
    // return Some(part2(input));
    None
}
