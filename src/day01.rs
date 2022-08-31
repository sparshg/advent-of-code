fn part1(input: &str) -> i32 {
    input
        .lines()
        .fold(0, |sum, line| sum + line.parse::<i32>().unwrap())
}

fn part2(input: &str) -> i32 {
    let input: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut sums: Vec<i32> = vec![0; input.len()];
    for a in 0..input.len() {
        for b in a..input.len() {
            sums[a] += input[b];
            if sums[a] == 0 {
                return sums[0];
            }
        }
    }
    loop {
        for a in &input {
            for x in 0..sums.len() {
                sums[x] += a;
                if sums[x] == 0 {
                    return sums[0];
                }
            }
        }
    }
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
