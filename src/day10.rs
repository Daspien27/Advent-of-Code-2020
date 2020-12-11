#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|s|{ s.parse().unwrap()}).collect()
}


#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
    let mut adaptors = input.clone();
    
    adaptors.sort();
    adaptors.insert(0, 0);

    let counts = adaptors.iter().zip(adaptors.iter().skip(1)).fold((0,0,1), |(diff1,diff2,diff3), (a, b)|{
        match b - a {
            1 => (diff1 + 1, diff2, diff3),
            2 => (diff1, diff2 + 1, diff3),
            3 => (diff1, diff2, diff3 + 1),
            _ => panic!("Aaah")
        }
    });

    counts.0 * counts.2
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
    let mut adaptors = input.clone();
    
    adaptors.sort();
    adaptors.insert(0, 0);
    adaptors.push(*adaptors.iter().last().unwrap() + 3);

    //println!("{:?}", adaptors);
    adaptors.iter().enumerate().zip(adaptors.iter().enumerate().skip(1)).scan(0usize, |start_of_next_sequence: &mut usize, ((ai, &a), (bi, &b))|{
        
        if b - a == 3 {
            let some = Some(Some((*start_of_next_sequence, ai)));
            *start_of_next_sequence = bi;
            return some;
        }

        Some(None)
    })
    .map(|x| {
        if let Some(range) = x {
            //bvprintln!("{:?}", &adaptors[range.0..=range.1]);
            match range.1 - range.0 {
                0|1 => 1,            //no elements in between may be permuted
                2 => 2,
                3 => 4,
                4 => 7,
                _ => panic!("aaaah")
            }
        }
        else {
            1
        }
    })
    .product::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_part2 (){
        let input : Vec<u64> = vec!{16,10,15,5,1,11,7,19,6,12,4};

        assert_eq!(solve_part2(&input), 8);
    }

    #[test]
    fn example2_part2 (){
        let input : Vec<u64> = vec!{28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3};

        assert_eq!(solve_part2(&input), 19208);
    }
}