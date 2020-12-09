#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|s|{ s.parse().unwrap()}).collect()
}


fn find_sum(n : u64, range : &Vec<&u64>) -> bool {
    for i in 0..(range.len() - 1) {
        for j in i..range.len() {
            let sum = range[i] + range[j];
            if sum == n {
                return true;
            }
        }
    }

    false
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
    input.iter().enumerate().skip(25).find_map(|(idx, n)| {
        let range: Vec<_> = input.iter().skip(idx - 25).take(25).collect();
        if find_sum(*n, &range) {
            None
        }
        else {
            Some(*n)
        }
    }).unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<u64>) -> Option<u64> {
    const STEP1_ERROR : u64 = 32321523;

    for i in 0..input.len() {
        for j in (i+2)..input.len() {
            if *&input[i..j].iter().sum::<u64>() == STEP1_ERROR {
                return  Some(**&input[i..j].iter().min().unwrap() + **&input[i..j].iter().max().unwrap());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

}