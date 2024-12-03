use itertools::Itertools;

fn part1(input: &str) -> i32 {
    input
        .split("mul(")
        .skip(1)
        .filter_map(|s| {
            let mut parts = s.split(')').next()?.split(',');
            let a = parts.next()?.parse::<i32>().ok()?;
            let b = parts.next()?.parse::<i32>().ok()?;
            Some(a * b)
        })
        .sum()
}
fn part2(input: &str) -> i32 {
    let mut iter = input.chars().peekable();
    let mut res = 0;
    let mut inc = 1;
    while let Some(c) = iter.next() {
        match c {
            'd' if iter.next_if_eq(&'o').is_some() => {
                let mut dos = 1;
                if iter.next_if_eq(&'n').is_some()
                    && iter.next_if_eq(&'\'').is_some()
                    && iter.next_if_eq(&'t').is_some()
                {
                    dos = 0;
                }
                if iter.next_if_eq(&'(').is_some() && iter.next_if_eq(&')').is_some() {
                    inc = dos;
                }
            }
            'm' if inc == 1
                && iter.next_if_eq(&'u').is_some()
                && iter.next_if_eq(&'l').is_some()
                && iter.next_if_eq(&'(').is_some() =>
            {
                let a = iter
                    .by_ref()
                    .take_while_ref(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if iter.next_if_eq(&',').is_some() {
                    let b = iter
                        .by_ref()
                        .take_while_ref(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    if iter.next_if_eq(&')').is_some() {
                        res += a * b;
                    }
                }
            }
            _ => {}
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
