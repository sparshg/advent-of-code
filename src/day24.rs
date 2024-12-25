use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct WireVal<'a> {
    val: Option<bool>,
    op: &'a str,
    in1: &'a str,
    in2: &'a str,
}

impl WireVal<'_> {
    fn new<'a>(val: Option<bool>, op: &'a str, in1: &'a str, in2: &'a str) -> WireVal<'a> {
        WireVal { val, op, in1, in2 }
    }
}

fn dfs(wire: &str, wires: &mut HashMap<&str, WireVal>) {
    if wires[wire].val.is_some() {
        return;
    }
    dfs(wires[wire].in1, wires);
    dfs(wires[wire].in2, wires);
    let in1 = wires[wires[wire].in1].val.unwrap();
    let in2 = wires[wires[wire].in2].val.unwrap();
    let val = match wires[wire].op {
        "AND" => in1 & in2,
        "OR" => in1 | in2,
        "XOR" => in1 ^ in2,
        _ => panic!(),
    };
    wires.get_mut(wire).unwrap().val = Some(val);
}

fn part1(input: &str) -> i32 {
    let (initial, rules) = input.split_once("\n\n").unwrap();
    let mut wires: HashMap<&str, WireVal> = HashMap::new();
    for inst in rules.lines() {
        let (in1, op, in2, _, w) = inst.split_whitespace().collect_tuple().unwrap();
        wires.insert(w, WireVal::new(None, op, in1, in2));
    }
    for line in initial.lines() {
        let (wire, value) = line.split_once(": ").unwrap();
        wires.insert(
            wire,
            WireVal::new(Some(value.parse::<u8>().unwrap() != 0), "", "", ""),
        );
    }
    for wire in wires.keys().cloned().collect_vec() {
        dfs(wire, &mut wires);
    }
    let mut res = 0;
    for i in 0.. {
        let Some(w) = wires.get(format!("z{:02}", i).as_str()) else {
            break;
        };
        res |= (w.val.unwrap() as u64) << i;
    }
    println!("{}", res);
    0
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
