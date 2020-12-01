
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32>
{
    input
        .lines()
        .map(|l| { l.parse().unwrap() })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut expense = input.to_vec();

    expense.sort();

    for e in &expense {

        let complement = 2020 - e;
        let find_complement = expense.binary_search(&complement);

        if let Ok(found) = find_complement
        {
            return e * expense[found];
        }
    }

    panic!("AOC lied to us!");
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut expense = input.to_vec();

    expense.sort();

    for (i, a) in expense.iter().enumerate() {
        let target = 2020 - a;

        for b in expense.iter().skip(i)
        {
            let complement = target - b;

            if complement > *b
            {
                let find_complement = expense.binary_search(&complement);

                if let Ok(found) = find_complement
                {
                    return a * b * expense[found];
                }
            }
        } 
    }

    panic!("AOC lied to us!");
}

#[cfg(test)]
mod tests {

}