use itertools::Itertools;

#[derive(Debug)]
struct Computer {
    ip: usize,
    registers: [u64; 3],
    instructions: Vec<(Instruction, u8)>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let (regs, program) = input.split_once("\n\n").unwrap();
        let registers = regs
            .lines()
            .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        let instructions = program
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .tuples()
            .map(|(a, b)| (a.into(), b))
            .collect();
        Self {
            ip: 0,
            registers,
            instructions,
        }
    }

    fn run(&mut self) -> String {
        let mut result = "".to_string();
        while let Some((instruction, op)) = self.instructions.get(self.ip) {
            if let Some(out) = instruction.eval(*op, &mut self.registers, &mut self.ip) {
                result.push((out + b'0') as char);
                result.push(',');
            }
        }
        result.pop();
        result
    }
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(op: u8) -> Self {
        match op {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl Instruction {
    fn eval(&self, op: u8, registers: &mut [u64; 3], ip: &mut usize) -> Option<u8> {
        *ip += 1;
        let combo = Operand::Combo(op).eval(registers);
        let literal = Operand::Literal(op).eval(registers);
        match self {
            Instruction::Adv => registers[0] /= 1 << combo,
            Instruction::Bxl => registers[1] ^= literal,
            Instruction::Bst => registers[1] = combo % 8,
            Instruction::Jnz if registers[0] != 0 => *ip = literal as usize / 2,
            Instruction::Bxc => registers[1] ^= registers[2],
            Instruction::Out => {
                return Some((combo % 8) as u8);
            }
            Instruction::Bdv => registers[1] = registers[0] / (1 << combo),
            Instruction::Cdv => registers[2] = registers[0] / (1 << combo),
            _ => (),
        };
        None
    }
}

enum Operand {
    Literal(u8),
    Combo(u8),
}

impl Operand {
    fn eval(&self, registers: &mut [u64; 3]) -> u64 {
        match self {
            Operand::Literal(v) => *v as u64,
            Operand::Combo(v) => match *v {
                x @ 0..4 => x as u64,
                x @ 4..7 => registers[x as usize - 4],
                _ => panic!("Invalid register"),
            },
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut computer = Computer::new(input);
    println!("{}", computer.run());
    0
}

fn part2(input: &str) -> i32 {
    let program = input
        .split_once("\n\n")
        .unwrap()
        .1
        .split_once(' ')
        .unwrap()
        .1;
    for i in 219000000870856.. {
        let mut computer = Computer::new(input);
        computer.registers[0] = i;
        let output = computer.run();
        println!("{} {} {}", i, output, program);
        if output == program {
            break;
        }
    }
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
