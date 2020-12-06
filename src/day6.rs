use regex::Regex;
use std::collections::{HashSet, HashMap};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"(?m)(?:(?:^.+$\n?)*)").unwrap();
    }
    
    RE.captures_iter(input)
        .map(|cap|{
            String::from(cap.get(0).unwrap().as_str())
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s|{
            s.chars().filter(|c| c.is_ascii_alphabetic()).collect::<HashSet<char>>().len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|s|{

            let mut map: HashMap<char, usize> = HashMap::new();

            s.lines().for_each(|s|{
                    s.chars()
                        .filter(|c| c.is_ascii_alphabetic())
                        .for_each(|c|{                    
                            let c = map.entry(c).or_insert(0);
                            *c += 1;
                        });
                });
            

            map
                .iter()
                .filter(|(_, val)| **val == s.lines().count())
                .count()
        })
        .sum()
}


#[cfg(test)]
mod tests {

}