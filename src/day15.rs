use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}


fn consider_last(last: u64, state: &HashMap<u64, (Option<usize>, Option<usize>)>) -> u64 {

    match state.get(&last).unwrap() {
        (Some(_), None) => 0,
        (Some(prev), Some(prev2)) => (prev - prev2) as u64,
        _ => panic!("Unexpected state.")
    }
}

fn play_memory_gane(input: &Vec<u64>, until: usize) -> u64 {
    
    let mut state : HashMap<u64, (Option<usize>, Option<usize>)> = HashMap::new();

    input.iter().enumerate().for_each(|(i,n)|{
        let e = state.entry(*n).or_insert((Some(i+1), None));
        *e = (Some(i+1), e.0);
    });

    let mut last = *input.last().unwrap();
    for i in (input.len()+1)..=until {
        let new_last = consider_last(last, &state);
        state.entry(new_last).and_modify(|e| {
            *e = (Some(i), e.0)
        }).or_insert((Some(i),None));
        last = new_last;
    }

    last
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
    play_memory_gane(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
    play_memory_gane(input, 30000000)
}


#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn part1_example_1(){
        assert_eq!(solve_part1(&vec![0,3,6]), 436);
    }

    #[test]
    fn part1_example_2(){
        assert_eq!(solve_part1(&vec![1,3,2]), 1);
    }

    #[test]
    fn part1_example_3(){
        assert_eq!(solve_part1(&vec![2,1,3]), 10);
    }

    #[test]
    fn part1_example_4(){
        assert_eq!(solve_part1(&vec![1,2,3]), 27);
    }

    #[test]
    fn part1_example_5(){
        assert_eq!(solve_part1(&vec![2,3,1]), 78);
    }

    #[test]
    fn part1_example_6(){
        assert_eq!(solve_part1(&vec![3,2,1]), 438);
    }

    #[test]
    fn part1_example_7(){
        assert_eq!(solve_part1(&vec![3,1,2]), 1836);
    }

        #[test]
    fn part2_example_1(){
        assert_eq!(solve_part2(&vec![0,3,6]), 175594);
    }

    #[test]
    fn part2_example_2(){
        assert_eq!(solve_part2(&vec![1,3,2]), 2578);
    }

    #[test]
    fn part2_example_3(){
        assert_eq!(solve_part2(&vec![2,1,3]), 3544142);
    }

    #[test]
    fn part2_example_4(){
        assert_eq!(solve_part2(&vec![1,2,3]), 261214);
    }

    #[test]
    fn part2_example_5(){
        assert_eq!(solve_part2(&vec![2,3,1]), 6895259);
    }

    #[test]
    fn part2_example_6(){
        assert_eq!(solve_part2(&vec![3,2,1]), 18);
    }

    #[test]
    fn part2_example_7(){
        assert_eq!(solve_part2(&vec![3,1,2]), 362);
    }
}