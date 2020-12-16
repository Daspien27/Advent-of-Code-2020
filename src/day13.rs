use num::Integer;

#[aoc_generator(day13, part1)]
pub fn input_generator_part1(input: &str) -> (u64, Vec<u64>) {
    
    let mut lines = input.lines();
    let earliest_to_depart = lines.next().unwrap().parse().unwrap();
    let bus_schedule = lines.next().unwrap().split(",").filter(|s| *s != "x").map(|c| c.parse().unwrap()).collect();

    (earliest_to_depart, bus_schedule)
}

#[aoc_generator(day13, part2)]
pub fn input_generator_part2(input: &str) -> Vec<(i128, i128)> {
    
    input
        .lines()
        .skip(1)
        .next().unwrap()
        .split(",")
        .enumerate()
        .filter(|(_,s)| *s != "x")
        .map(|(i, c)| (i as i128, c.parse().unwrap()))
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1((earliest_to_depart, bus_schedule): &(u64, Vec<u64>)) -> u64 {
    
    let minutes_after_earliest_departure = |bus : u64|{
        bus - (earliest_to_depart % bus)
    };
    
    let min = bus_schedule.iter().min_by_key(|bus|{
        minutes_after_earliest_departure(**bus)
    });

    min.unwrap() * minutes_after_earliest_departure(*min.unwrap())
}

#[aoc(day13, part2)]
pub fn solve_part2(bus_schedule: &Vec<(i128, i128)>) -> i128 {
    
    let target = bus_schedule
        .iter()
        .map(|(i, b)| ((b - i) % b, *b))
        .fold_first(|(t, mod_g), (a, mod_n)|{
            let (e, lcm) = mod_g.extended_gcd_lcm(&mod_n); // we need bezout coefficients for the chinese remainder theorem
            let x = (t * mod_n * e.y + a * mod_g * e.x) / e.gcd;

            ((x % lcm + lcm) % lcm, lcm)
        }).unwrap();

    (target.0 + target.1) % target.1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part2_example1(){
        let input = input_generator_part2(".\n7,13,x,x,59,x,31,19");
        assert_eq!(solve_part2(&input), 1068781);
    }

    #[test]
    fn test_part2_example2(){
        let input = input_generator_part2(".\n17,x,13,19");
        assert_eq!(solve_part2(&input), 3417);
    }

    #[test]
    fn test_part2_example3(){
        let input = input_generator_part2(".\n67,7,59,61");
        assert_eq!(solve_part2(&input), 754018);
    }

    #[test]
    fn test_part2_example4(){
        let input = input_generator_part2(".\n67,x,7,59,61");
        assert_eq!(solve_part2(&input), 779210);
    }

    #[test]
    fn test_part2_example5(){
        let input = input_generator_part2(".\n67,7,x,59,61");
        assert_eq!(solve_part2(&input), 1261476);
    }

    #[test]
    fn test_part2_example6(){
        let input = input_generator_part2(".\n1789,37,47,1889");
        assert_eq!(solve_part2(&input), 1202161486);
    }

}