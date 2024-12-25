fn next(mut x: u32) -> u32 {
    x ^= x << 6;
    x &= 16777215;
    x ^= x >> 5;
    x &= 16777215;
    x ^= x << 11;
    x &= 16777215;
    x
}
fn next2(mut x: u32) -> u32 {
    x ^= x << 6;
    x &= 16777215;
    x ^= x >> 5;
    x &= 16777215;
    x ^= x << 11;
    x &= 16777215;
    x
}

fn part1(input: &str) -> i32 {
    let res = input
        .lines()
        .map(|x| {
            let mut x = x.parse::<u32>().unwrap();
            for _ in 0..2000 {
                x = next(x);
            }
            x as u64
        })
        .sum::<u64>();
    println!("{}", res);
    0
}

fn part2(input: &str) -> i32 {
    let res = input
        .lines()
        .map(|x| {
            let mut x = x.parse::<u32>().unwrap();
            for _ in 0..10 {
                x = next2(x);
                println!("{}", x % 10);
            }
            x as u64
        })
        .sum::<u64>();
    println!("{}", res);
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
