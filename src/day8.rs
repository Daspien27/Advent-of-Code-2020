use regex::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Instruction
{
    Acc(i64),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug, Clone)]
pub struct Console {
    accumulator : i64,
    instruction_ptr : isize,
    program : Vec<Instruction>,
}

impl Console {

    fn run_part1 (&mut self) -> i64 {

        let mut instructions_proccessed : HashSet<isize> = HashSet::new();

        loop {

            if instructions_proccessed.contains(&self.instruction_ptr)
            {
                return self.accumulator;
            }
            else
            {
                instructions_proccessed.insert(self.instruction_ptr);
            }
            
            match self.program[self.instruction_ptr as usize] {
                Instruction::Acc(val) => {
                    self.accumulator += val;
                    self.instruction_ptr += 1;
                },
                Instruction::Jmp(val) => {
                    self.instruction_ptr += val;
                }

                Instruction::Nop(_) => {
                    self.instruction_ptr += 1;
                }
            }
        }
    }

    fn run_part2 (&mut self) -> Option<i64> {

        let mut instructions_proccessed : HashSet<isize> = HashSet::new();

        loop {

            if self.instruction_ptr == self.program.len() as isize {
                break;
            }

            if instructions_proccessed.contains(&self.instruction_ptr)
            {
                return None;
            }
            else
            {
                instructions_proccessed.insert(self.instruction_ptr);
            }
            
            match self.program[self.instruction_ptr as usize] {
                Instruction::Acc(val) => {
                    self.accumulator += val;
                    self.instruction_ptr += 1;
                },
                Instruction::Jmp(val) => {
                    self.instruction_ptr += val;
                }

                Instruction::Nop(_) => {
                    self.instruction_ptr += 1;
                }
            }
        }

        Some(self.accumulator)
    }


}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Console {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"(?P<instruction>acc|nop|jmp) (?P<n>[+-]\d+)?").unwrap();
    }

    Console { accumulator: 0, instruction_ptr: 0, program: 
    RE.captures_iter(input).map(|cap|{

        match cap.name("instruction").unwrap().as_str() {
            "acc" => Instruction::Acc(cap.name("n").unwrap().as_str().parse().unwrap()),
            "jmp" => Instruction::Jmp(cap.name("n").unwrap().as_str().parse().unwrap()),
            "nop" => Instruction::Nop(cap.name("n").unwrap().as_str().parse().unwrap()),
            _ => panic!("Aaaaah!")
        }
    })
    .collect() }
}


#[aoc(day8, part1)]
pub fn solve_part1(input: &Console) -> i64 {
    let mut main_console = input.clone();

    main_console.run_part1()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Console) -> i64 {
    
    (0..input.program.len()).find_map(|i|{
        let mut main_console = input.clone();

        match main_console.program[i] {
            Instruction::Jmp(val) => main_console.program[i] = Instruction::Nop(val),
            Instruction::Nop(val) => main_console.program[i] = Instruction::Jmp(val),
            _ => return None
        }

        main_console.run_part2()
    }).unwrap()
}

#[cfg(test)]
mod tests {

}