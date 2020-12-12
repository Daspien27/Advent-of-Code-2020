use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction
{
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Forward(i64),
    Left(i64),
    Right(i64),
}


#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref INSTRUCTION_RE : Regex = Regex::new(r"(?P<instruction>[NSEWLRF])(?P<value>\d+)").unwrap();
    }
    
    INSTRUCTION_RE.captures_iter(input)
        .map(|cap|{

            let amount = cap.name("value").unwrap().as_str().parse().unwrap();

            match cap.name("instruction").unwrap().as_str() {
                "N" => Instruction::North(amount),
                "S" => Instruction::South(amount),
                "E" => Instruction::East(amount),
                "W" => Instruction::West(amount),
                "L" => Instruction::Left(amount),
                "R" => Instruction::Right(amount),
                "F" => Instruction::Forward(amount),
                _ => panic!("Unexpected instruction!")
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> i64 {
    
    let trip = input.iter().fold((0, 0, 0), |(distance_x, distance_y, angle), instruction|{
        match instruction {
            Instruction::North(value) => (distance_x, distance_y + value, angle),
            Instruction::South(value) => (distance_x, distance_y - value, angle),
            Instruction::East(value) => (distance_x + value, distance_y, angle),
            Instruction::West(value) => (distance_x - value, distance_y, angle),
            Instruction::Left(value) => (distance_x, distance_y, (angle + value + 360) % 360),
            Instruction::Right(value) => (distance_x, distance_y, (angle - value + 360) % 360),
            Instruction::Forward(value) => {
                match angle {
                    0 => (distance_x + value, distance_y, angle),
                    90 => (distance_x, distance_y + value, angle),
                    180 => (distance_x - value, distance_y, angle),
                    270 => (distance_x, distance_y - value, angle),
                    _ => panic!("{} is an unexpected angle!", angle)
                }

            }
        }
    });

    trip.0.abs() + trip.1.abs()
}

#[derive(Clone, Copy)]
struct Part2Model {
    ship : (i64, i64),
    waypoint : (i64, i64)
}

impl Part2Model {
    fn new() -> Self {
        Part2Model{ship: (0,0), waypoint: (10, 1)}
    }

    fn rotate_waypoint(&mut self, angle: i64) {
        match (angle + 360) % 360 {
            0 => (),
            // https://en.wikipedia.org/wiki/Rotation_matrix#Common_rotations
            90 =>  self.waypoint = (-self.waypoint.1, self.waypoint.0),
            180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
            270 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            _ => panic!("{} is an unexpected angle!", angle)
        }
    }

    fn apply_instruction(mut self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::North(value) => self.waypoint.1 += value,
            Instruction::South(value) => self.waypoint.1 -= value,
            Instruction::East(value) =>  self.waypoint.0 += value,
            Instruction::West(value) =>  self.waypoint.0 -= value,
            Instruction::Left(value) =>  self.rotate_waypoint(*value),
            Instruction::Right(value) => self.rotate_waypoint(-1 * *value),
            Instruction::Forward(value) => {
                self.ship.0 += value * self.waypoint.0;
                self.ship.1 += value * self.waypoint.1;

            }
        }

        self
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> i64 {
    let trip = input.iter().fold(Part2Model::new(), |model, instruction|{
        model.apply_instruction(&instruction)
    });

    trip.ship.0.abs() + trip.ship.1.abs()
}


#[cfg(test)]
mod tests {
 
}