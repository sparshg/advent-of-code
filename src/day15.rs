use std::collections::VecDeque;

fn hash(x: &str) -> u8 {
    x.chars()
        .fold(0u8, |h, c| h.wrapping_add(c as u8).wrapping_mul(17))
}

fn part1(input: &str) -> i32 {
    input
        .split(',')
        .fold(0, |acc, curr| acc + hash(curr) as i32)
}

fn part2(input: &str) -> i32 {
    let mut boxes: Vec<VecDeque<(&str, i32)>> = vec![VecDeque::new(); 256];
    for instr in input.split(',') {
        if instr.ends_with('-') {
            let label = &instr[..instr.len() - 1];
            boxes[hash(label) as usize].retain(|&(x, _)| x != label);
        } else {
            let (label, num) = instr.split_once('=').unwrap();
            let num = num.parse::<i32>().unwrap();
            let b = &mut boxes[hash(label) as usize];
            for (l, v) in b.iter_mut() {
                if l == &label {
                    *v = num;
                }
            }
            if !b.contains(&(label, num)) {
                b.push_back((label, num));
            }
        }
    }
    boxes.iter().enumerate().fold(0, |acc, (i, b)| {
        acc + b
            .iter()
            .enumerate()
            .fold(0, |a, (j, v)| a + (i as i32 + 1) * (j as i32 + 1) * v.1)
    })
}
#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
