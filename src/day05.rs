use std::cmp::min;

fn part1(input: &str) -> i32 {
    let mut ans = "".to_string();
    for char in input.chars() {
        if let Some(last) = ans.chars().last() {
            if last.to_lowercase().to_string() == char.to_lowercase().to_string()
                && last.is_lowercase() != char.is_lowercase()
            {
                ans.pop();
                continue;
            }
        }
        ans.push(char);
    }
    ans.len() as i32
}

fn part2(input: &str) -> i32 {
    let mut ans = "".to_string();
    let mut best = i32::MAX;
    for curr in 'a'..='z' {
        for char in input.chars() {
            if char.to_lowercase().to_string() == curr.to_string() {
                continue;
            }
            if let Some(last) = ans.chars().last() {
                if last.to_lowercase().to_string() == char.to_lowercase().to_string()
                    && last.is_lowercase() != char.is_lowercase()
                {
                    ans.pop();
                    continue;
                }
            }
            ans.push(char);
        }
        best = min(best, ans.len() as i32);
        ans = "".to_string();
    }
    best
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
