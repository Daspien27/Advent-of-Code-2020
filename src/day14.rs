use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
enum Instruction
{
    Mask(String),
    Mem(usize, u64)
}

#[derive(Debug, Clone)]
pub struct Program
{
    instructions: Vec<Instruction>,
    mask: String,
    memory: HashMap<usize, u64>,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Program {
    lazy_static! {
        static ref INSTRUCTION_RE : Regex = Regex::new(r"(?P<instruction>mem|mask)(:?(:?\[(?P<address>\d+)\] = (?P<value>\d+))|(:? = (?P<mask>.+)))").unwrap();
    }
    
    let instructions = INSTRUCTION_RE.captures_iter(input)
        .map(|cap|{
            match cap.name("instruction").unwrap().as_str() {
                "mem" => Instruction::Mem(cap.name("address").unwrap().as_str().parse().unwrap(), cap.name("value").unwrap().as_str().parse().unwrap()),
                "mask" => Instruction::Mask(cap.name("mask").unwrap().as_str().chars().rev().collect()),
                _=> panic!("Unexpected instruction!")
            }
        })
        .collect();

    Program{instructions: instructions, mask: String::new(), memory: HashMap::new()}
}

fn apply_mask (mask: &str, mut val: u64) -> u64 {
    
    mask.chars().enumerate().for_each(|(i, c)|{
        match c {
            '0' => val = val & !(1<<i),
            '1' => val = val | (1<<i),
            'X' => (),
            _ => panic!("Unexpected bit string value!")
        };
    });
    
    val
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Program) -> u64 {
    
    let mut main_program = input.clone();

    input.instructions.iter().for_each(|i|{
        match i {
            Instruction::Mask(m) => {
                main_program.mask = m.clone();
            },
            Instruction::Mem(key, val) => {
                main_program.memory.insert(*key, apply_mask(main_program.mask.as_str(), *val));
            },
        };
    });

    main_program.memory.iter().map(|(_, v)| v).sum()
}

fn addr_from_floating(mask: &str, addr: usize, floating_addr: usize) -> usize {
    mask.chars().enumerate().fold((0usize, 0), |(acc, fi), (i, c)|{
        match c {
            '0' => (acc | ((1<<i) & addr), fi),
            '1' => (acc | ((1<<i)), fi),
            'X' => (acc | ((1<<i) & (((1<<fi) & floating_addr)<<(i-fi))), fi + 1),
            _ =>  panic!("Unexpected bit mask character!")
        }
    }).0
}

fn apply_memory_mask(mask: &str, addr: usize) -> Vec<usize> {

    let floating_count = mask.chars().filter(|c| *c == 'X').count();

    let floating_addr : Vec<usize> = (0..(1<<floating_count)).collect();

    floating_addr.iter().map(|a|{
        addr_from_floating(&mask, addr, *a)
    }).collect()

}


#[aoc(day14, part2)]
pub fn solve_part2(input: &Program) -> u64 {
    
    let mut main_program = input.clone();

    input.instructions.iter().for_each(|i|{
        match i {
            Instruction::Mask(m) => {
                main_program.mask = m.clone();
            },
            Instruction::Mem(key, val) => {
                let keys = apply_memory_mask(main_program.mask.as_str(), *key);
                keys.iter().for_each(|k| {
                    main_program.memory.insert(*k, *val);
                });
            },
        };
    });

    main_program.memory.iter().map(|(_, v)| v).sum()
}

#[cfg(test)]
mod tests {

}