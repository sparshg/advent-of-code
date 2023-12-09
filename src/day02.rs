use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input.lines().enumerate().fold(0, |sum, (i, line)| {
        sum + if line.split_once(':').unwrap().1.split(&[';', ',']).any(|x| {
            let (n, c) = x.trim().split_once(' ').unwrap();
            n.parse::<i32>().unwrap() > colors[c]
        }) {
            0
        } else {
            i as i32 + 1
        }
    })
}

fn part2(input: &str) -> i32 {
    input.lines().fold(0, |sum, line| {
        let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        line.split_once(':')
            .unwrap()
            .1
            .split(&[';', ','])
            .for_each(|x| {
                let (n, c) = x.trim().split_once(' ').unwrap();
                colors.insert(c, n.parse::<i32>().unwrap().max(colors[c]));
            });
        sum + colors.values().product::<i32>()
    })
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
