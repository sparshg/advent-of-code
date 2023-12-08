use std::i32;



fn part1(input: &str) -> i32 {
    input.lines().fold(0, |sum, line| {
        sum + line
            .chars()
            .find(|&x| x.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
            * 10
            + line
                .chars()
                .rev()
                .find(|&x| x.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap() as i32
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
