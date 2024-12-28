use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Register {
    pub value: u64,
}

impl Register {
    const fn new(initial_value: u64) -> Register {
        Register {
            value: initial_value,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Operand {
    literal_value: u8,
}

impl Operand {
    fn new(literal_value: u8) -> Operand {
        Operand { literal_value }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    opcode: u8,
    operand: Operand,
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: Register,
    b: Register,
    c: Register,
}

impl Registers {
    fn new(initial_a: u64, initial_b: u64, initial_c: u64) -> Registers {
        Registers {
            a: Register::new(initial_a),
            b: Register::new(initial_b),
            c: Register::new(initial_c),
        }
    }

    fn read_value(&self, operand: Operand) -> u64 {
        match operand.literal_value {
            4 => self.a.value,
            5 => self.b.value,
            6 => self.c.value,
            _ => operand.literal_value as u64,
        }
    }
}

fn to_instructions(raw_instructions: Vec<u8>) -> Vec<Instruction> {
    raw_instructions
        .chunks(2)
        .map(|chunk| Instruction {
            opcode: chunk[0],
            operand: Operand::new(chunk[1]),
        })
        .collect()
}

fn load_input(path: &str) -> (Registers, Vec<Instruction>) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut initial_a: u64 = 0;
    let mut initial_b: u64 = 0;
    let mut initial_c: u64 = 0;

    let register_re = Regex::new(r"Register [A-C]: (\d+)").unwrap();
    let program_re = Regex::new(r"Program: (.+)").unwrap();
    let file = File::open(path).expect("file not found!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = register_re.captures(&line) {
            let value: u64 = caps[1].parse().unwrap();
            if line.contains("Register A") {
                initial_a = value;
            } else if line.contains("Register B") {
                initial_b = value;
            } else if line.contains("Register C") {
                initial_c = value;
            }
        } else if let Some(caps) = program_re.captures(&line) {
            instructions = to_instructions(
                caps[1]
                    .split(',')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|parsed| parsed.parse::<u8>().unwrap())
                    .collect(),
            );
        }
    }

    (
        Registers::new(initial_a, initial_b, initial_c),
        instructions,
    )
}
struct Executor<'a> {
    instruction_pointer: usize,
    outputs: Vec<u8>,
    registers: &'a mut Registers,
}

impl<'a> Executor<'a> {
    fn new(registers: &'a mut Registers) -> Executor<'a> {
        Executor {
            instruction_pointer: 0,
            outputs: Vec::new(),
            registers,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            0 => self.adv(instruction.operand),
            1 => self.bxl(instruction.operand),
            2 => self.bst(instruction.operand),
            3 => self.jnz(instruction.operand),
            4 => self.bxc(instruction.operand),
            5 => self.out(instruction.operand),
            6 => self.bdv(instruction.operand),
            7 => self.cdv(instruction.operand),
            _ => panic!("Unknown opcode"),
        }
        self.instruction_pointer += 1;
    }

    fn adv(&mut self, operand: Operand) {
        self.registers.a.value /= 2u64.pow(self.registers.read_value(operand) as u32);
    }

    fn bxl(&mut self, operand: Operand) {
        self.registers.b.value ^= operand.literal_value as u64;
    }

    fn bst(&mut self, operand: Operand) {
        self.registers.b.value = self.registers.read_value(operand) % 8;
    }

    fn jnz(&mut self, operand: Operand) {
        if self.registers.a.value != 0 {
            self.instruction_pointer = (operand.literal_value / 2) as usize - 1;
        }
    }

    fn bxc(&mut self, _: Operand) {
        self.registers.b.value ^= self.registers.c.value;
    }

    fn out(&mut self, operand: Operand) {
        self.outputs
            .push((self.registers.read_value(operand) % 8) as u8);
    }

    fn bdv(&mut self, operand: Operand) {
        self.registers.b.value =
            self.registers.a.value / 2u64.pow(self.registers.read_value(operand) as u32);
    }
    fn cdv(&mut self, operand: Operand) {
        self.registers.c.value =
            self.registers.a.value / 2u64.pow(self.registers.read_value(operand) as u32);
    }
}

fn exectute(mut registers: Registers, instructions: &[Instruction]) -> Vec<u8> {
    let mut executor = Executor::new(&mut registers);

    while executor.instruction_pointer < instructions.len() {
        executor.execute(instructions[executor.instruction_pointer])
    }

    executor.outputs
}

fn solution_1(registers: Registers, instructions: Vec<Instruction>) -> String {
    exectute(registers, &instructions)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn find_output_matches_instructions(instructions: Vec<Instruction>) -> u64 {
    let mut raw_instructions = Vec::new();

    for instruction in instructions.iter() {
        raw_instructions.push(instruction.opcode);
        raw_instructions.push(instruction.operand.literal_value);
    }

    let mut initial_a = 0;

    loop {
        initial_a += 1;
        if initial_a % 100000000 == 0 {
            println!("Trying a: {}", initial_a);
        }
        let mut registers = Registers::new(initial_a, 0, 0);
        let mut executor = Executor::new(&mut registers);
        while executor.instruction_pointer < instructions.len() {
            executor.execute(instructions[executor.instruction_pointer]);
            let last_output_index = executor.outputs.len() - 1;
            if !executor.outputs.is_empty()
                && raw_instructions[last_output_index] != executor.outputs[last_output_index]
            {
                break;
            }
        }
        if executor.outputs.len() == raw_instructions.len() {
            return initial_a;
        }
    }
}

fn solution_2(instructions: Vec<Instruction>) -> u64 {
    find_output_matches_instructions(instructions)
}

fn main() {
    let input_start = Instant::now();
    let (registers, instructions) = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());
    println!("{:?}, {:?}", registers, instructions);

    let solution_1_start = Instant::now();
    let output_1 = solution_1(registers.clone(), instructions.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );
    let solution_2_start = Instant::now();
    let output_2 = solution_2(instructions.clone());

    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
