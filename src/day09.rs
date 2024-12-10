use itertools::Itertools;

#[derive(Debug, Default)]
struct Disk {
    id: usize,
    size: usize,
}

fn parse(input: &str) -> Vec<Disk> {
    input
        .char_indices()
        .map(|(id, size)| Disk {
            id: id / 2,
            size: size.to_digit(10).unwrap() as usize,
        })
        .collect_vec()
}

fn part1(input: &str) -> i32 {
    let mut disk = parse(input);
    let (mut i, mut j) = (0, disk.len() - 1);
    let mut res = Vec::new();
    while i < j {
        if i % 2 == 0 {
            res.extend((0..disk[i].size).map(|x| disk[i].id));
            i += 1;
        }
        if disk[j].size > disk[i].size {
            res.extend((0..disk[i].size).map(|x| disk[j].id));
            disk[j].size -= disk[i].size;
            i += 1;
        } else {
            res.extend((0..disk[j].size).map(|x| disk[j].id));
            disk[i].size -= disk[j].size;
            j -= 2;
        }
    }
    res.extend((0..disk[j].size).map(|x| disk[j].id));

    println!(
        "{}",
        res.iter().enumerate().fold(0, |acc, (i, &x)| acc + x * i)
    );
    0
}

fn part2(input: &str) -> i32 {
    let mut disk = parse(input);
    let mut i = 0;
    let mut res = Vec::new();
    'l: while i < disk.len() {
        if i % 2 == 0 {
            res.extend((0..disk[i].size).map(|x| disk[i].id));
            i += 1;
        }
        for k in (i + 1..disk.len()).rev().step_by(2) {
            if (1..=disk[i].size).contains(&disk[k].size) && disk[k].id > 0 {
                res.extend((0..disk[k].size).map(|x| disk[k].id));
                disk[i].size -= disk[k].size;
                disk[k].id = 0;
                continue 'l;
            }
        }
        if let Some(disk) = disk.get(i) {
            res.extend((0..disk.size).map(|_| 0));
        }
        i += 1;
    }
    println!(
        "{}",
        res.iter().enumerate().fold(0, |acc, (i, &x)| acc + x * i)
    );
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
