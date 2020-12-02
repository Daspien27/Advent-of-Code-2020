
use regex::Regex;

#[derive(Debug)]
pub struct Password
{
    bound: (usize, usize),
    limit: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| { 
            let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

            let cap = re.captures_iter(l).nth(0).unwrap(); //just the first match

            Password{bound: (cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap()), 
                        limit: cap.get(3).unwrap().as_str().chars().nth(0).unwrap(), 
                        password: String::from(cap.get(4).unwrap().as_str()) }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> usize {

      input
        .iter()
        .filter(|&p| {
            let count = p.password.chars().filter(|&c| c == p.limit).count();
            p.bound.0 <=  count && count <= p.bound.1
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> usize {

      input
        .iter()
        .filter(|&p| {
            (p.password.chars().nth(p.bound.0 - 1).unwrap() == p.limit) ^ (p.password.chars().nth(p.bound.1 - 1).unwrap() == p.limit)
        })
        .count()
}

#[cfg(test)]
mod tests {

}