#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| String::from(l))
        .collect()
}


fn partition_row(partition: &str) -> u64
{
    partition
        .chars()
        .take(7)
        .fold((0u64, 127u64), |acc, c|
        {
            let diff = (acc.1 - acc.0) / 2 + (acc.1 - acc.0) % 2;

            match c
            {
                'F' => (acc.0, acc.1 - diff),
                'B' => (acc.0 + diff, acc.1),
                _ => panic!("Scary input!")
            }
        }).0
}

fn partition_col(partition: &str) -> u64
{
    partition
        .chars()
        .skip(7)
        .take(3)
        .fold((0u64, 7u64), |acc, c|
        {
            let diff = (acc.1 - acc.0) / 2 + (acc.1 - acc.0) % 2;

            match c
            {
                'L' => (acc.0, acc.1 - diff),
                'R' => (acc.0 + diff, acc.1),
                _ => panic!("Scary input!")
            }
        }).1
}

fn calculate_seat(partition: &str) -> u64
{
    let row = partition_row (partition);
    let col = partition_col(partition);

    row * 8 + col
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<String>) -> u64 {
     input
        .iter()
        .map(|p| calculate_seat(p))
        .max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<String>) -> u64 {
     
    let mut seat_ids : Vec<u64> = input
        .iter()
        .map(|p| calculate_seat(p.as_str()))
        .collect();

    seat_ids.sort();
        
    seat_ids
        .iter()
        .skip(1)
        .zip(seat_ids.iter())
        .find_map(|p|{
            if p.0 - p.1 > 1 { Some(p.1 + 1) } else { None }
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // FBFBBFFRLR
    fn example_1_row() {
        assert!(partition_row("FBFBBFFRLR") ==  44);
    }

    #[test]
    fn example_1_col() {
        assert!(partition_col("FBFBBFFRLR") ==  5);
    }

    #[test]
    fn example_1_calculate() {
        assert!(calculate_seat("FBFBBFFRLR") ==  357);
    }

    #[test]
    fn example_2_calculate() {
        assert!(calculate_seat("BFFFBBFRRR") ==  567);
    }

    #[test]
    fn example_3_calculate() {
        assert!(calculate_seat("FFFBBBFRRR") ==  119);
    }

    #[test]
    fn example_4_calculate() {
        assert!(calculate_seat("BBFFBBFRLL") ==  820);
    }
}