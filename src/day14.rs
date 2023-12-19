use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let input = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let mut total: i32 = 0;
    for i in 0..input[0].len() {
        let mut balls = 0;
        for j in (0..input.len()).rev() {
            match input[j][i] {
                'O' => balls += 1,
                '#' => {
                    total += ((input.len() - j - balls)..(input.len() - j))
                        .sum1()
                        .unwrap_or(0) as i32;
                    balls = 0;
                }
                _ => {}
            }
        }
        total += ((input.len() - balls + 1)..(input.len() + 1))
            .sum1()
            .unwrap_or(0) as i32;
    }
    total
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
