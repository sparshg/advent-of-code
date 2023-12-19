use itertools::Itertools;

fn minimize(
    l: i32,
    d: i32,
    i: i32,
    j: i32,
    input: &Vec<Vec<u32>>,
    memo: &mut Vec<Vec<Vec<Vec<Option<u32>>>>>,
    hist: &mut Vec<Vec<bool>>,
) -> u32 {
    if d < 0 || i < 0 || j < 0 || d > 3 || l < 0 {
        return u32::MAX;
    }
    let (l, d, i, j) = (l as usize, d as usize, i as usize, j as usize);
    if i > input.len() - 1 || j > input[0].len() - 1 || hist[i][j] {
        return u32::MAX;
    }
    if i == input.len() - 1 && j == input[0].len() - 1 {
        return input[i][j];
    }
    hist[i][j] = true;
    if memo[l][d][i][j].is_none() {
        // - 0 -
        // 1 * 3
        // - 2 -
        let mut dist = u32::MAX;
        for x in 0..4 {
            if d as i32 != (x + 2) % 4 {
                dist = (input[i][j].saturating_add(minimize(
                    if d as i32 == x { l as i32 - 1 } else { 2 },
                    x,
                    i as i32 + (x - 1) % 2,
                    j as i32 + (x - 2) % 2,
                    input,
                    memo,
                    hist,
                )))
                .min(dist);
            }
        }
        memo[l][d][i][j] = Some(dist);
    }
    hist[i][j] = false;
    memo[l][d][i][j].unwrap()
}

fn part1(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let mut memo = vec![vec![vec![vec![None::<u32>; input[0].len()]; input.len()]; 4]; 3];
    let mut hist = vec![vec![false; input[0].len()]; input.len()];
    (minimize(2, 2, 0, 0, &input, &mut memo, &mut hist)
        .min(minimize(2, 3, 0, 0, &input, &mut memo, &mut hist))
        - input[0][0]) as i32
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
