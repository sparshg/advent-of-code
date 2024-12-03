use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec()
    })
}

fn part1(input: &str) -> i32 {
    parse(input).fold(0, |acc, line| {
        let increasing = if line[1] > line[0] { 1 } else { -1 };
        if line
            .into_iter()
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&((b - a) * increasing)))
        {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> i32 {
    parse(input).enumerate().fold(0, |acc, (i, mut line)| {
        let increasing = (0..4)
            .tuple_windows()
            .map(|(a, b)| (line[b] - line[a]).signum())
            .sum::<i32>()
            .signum();

        let mut mutated = false;
        for i in 1..line.len() - 1 {
            if (line[i] - line[i - 1]) * increasing <= 0
                || (line[i] - line[i + 1]) * increasing >= 0
            {
                line.remove(i + 1);
                mutated = true;
                break;
            }
        }
        if i == 627 {
            println!("{:?}", line);
            println!("{:?}", mutated);
        }
        if !mutated {
            if (line[0] - line[1]) * increasing >= 0
                || (line[0] - line[2]) * increasing <= 0 && (line[1] - line[0]) * increasing > 3
            {
                line.remove(0);
            } else {
                line.pop();
            }
        }
        if i == 627 {
            println!("{:?}", line);
            println!("{:?}", mutated);
        }

        if line
            .into_iter()
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&((b - a) * increasing)))
        {
            print!("{}: {:?}", i + 1, increasing);

            println!(" {:?}", 1);
            acc + 1
        } else {
            // println!(" {:?}", 0);
            acc
        }
    })
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
