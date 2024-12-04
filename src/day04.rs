use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let mut res = 0;
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let word = "XMAS";
    for r in 0..lines.len() as i32 {
        for c in 0..lines[0].len() as i32 {
            for &(dr, dc) in &dirs {
                if word.chars().enumerate().all(|(i, w)| {
                    let nr = (r + dr * i as i32) as usize;
                    let nc = (c + dc * i as i32) as usize;
                    lines.get(nr).and_then(|line| line.get(nc)) == Some(&w)
                }) {
                    res += 1;
                }
            }
        }
    }
    res
}

fn part2(input: &str) -> i32 {
    let mut res = 0;
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let dirs = [((1, 1), (-1, -1)), ((-1, 1), (1, -1))];
    for r in 0..lines.len() {
        for c in 0..lines[0].len() {
            if lines[r][c] == 'A'
                && dirs.iter().all(|(a, b)| {
                    let (r1, c1) = ((r as i32 + a.0) as usize, (c as i32 + a.1) as usize);
                    let (r2, c2) = ((r as i32 + b.0) as usize, (c as i32 + b.1) as usize);
                    let w1 = lines.get(r1).and_then(|line| line.get(c1));
                    let w2 = lines.get(r2).and_then(|line| line.get(c2));
                    w1 == Some(&'M') && w2 == Some(&'S') || w1 == Some(&'S') && w2 == Some(&'M')
                })
            {
                res += 1;
            }
        }
    }
    res
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
