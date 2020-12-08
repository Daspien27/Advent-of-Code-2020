use regex::Regex;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Bag {
    contents : Option<Vec<(u64, String)>>
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String,Bag> {
    lazy_static! {
        static ref BAGS_RE : Regex = Regex::new(r"(?P<bag_type>\w+ \w+) bags contain (?P<contents>(:?(:?(:?\d+ \w+ \w+ bags?)(:?, )?)+)|(:?no other bags))").unwrap();
        static ref CONTENTS_RE : Regex = Regex::new(r"(?P<quantity>\d+) (?P<bag_type>\w+ \w+) bags?").unwrap();
    }
    
    BAGS_RE.captures_iter(input)
        .map(|cap|{
            let bag_type = String::from(cap.name("bag_type").unwrap().as_str());
            let contents_str = cap.name("contents").unwrap().as_str();

            match contents_str
            {
                "no other bags" => (bag_type, Bag{contents: None}),
                _ => {

                    let content : Vec<_> = CONTENTS_RE.captures_iter(contents_str)
                        .map(|cap|{
                            (cap.name("quantity").unwrap().as_str().parse().unwrap(), String::from(cap.name("bag_type").unwrap().as_str()))
                        })
                        .collect();
                                        
                    (bag_type, Bag{contents : Some(content)})
                }
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, Bag>) -> u64 {
    
    let mut can_contain_shiny_gold_bag: HashMap<&str, Option<bool>> = input.iter().map(|(k, _v)| (k.as_str(), None)).collect();
    
    *can_contain_shiny_gold_bag.get_mut("shiny gold").unwrap() = Some(true);

    while can_contain_shiny_gold_bag.iter().find(|(_, v)| v.is_none()).is_some()
    {
        input.iter().for_each(|(k, v)| {

            if can_contain_shiny_gold_bag.get(k.as_str()).unwrap().is_none() {

                let can_contain = match &v.contents {
                    Some(contents) => {
                        if contents.iter().any(|(_, bag_type)|{
                            Some(true) == *can_contain_shiny_gold_bag.get(bag_type.as_str()).unwrap()
                        }) {
                            Some(true)
                        }
                        else if contents.iter().all(|(_, bag_type)|{
                            Some(false) == *can_contain_shiny_gold_bag.get(bag_type.as_str()).unwrap()
                        })
                        {
                            Some(false)
                        }
                        else
                        {
                            return;
                        }
                    },
                    None => {
                        Some(false)
                    }
                };

                *can_contain_shiny_gold_bag.get_mut(k.as_str()).unwrap() = can_contain;
            }
        });
    }

    can_contain_shiny_gold_bag.iter().map(|(_, c)| -> u64 {
        match c {
            Some(true) => 1,
            Some(false) => 0,
            None => panic!("Bad code")
        }
    })
    .sum::<u64>() - 1
}


#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, Bag>) -> u64 {
    
    let mut amount_of_nested_bags: HashMap<&str, Option<u64>> = input.iter().map(|(k, _v)| (k.as_str(), None)).collect();

    while amount_of_nested_bags.get("shiny gold").unwrap().is_none()
    {
        input.iter().for_each(|(k, v)| {

            if amount_of_nested_bags.get(k.as_str()).unwrap().is_none() {

                let amount_nested = match &v.contents {
                    Some(contents) => {
                        
                        contents.iter().fold(Some(0u64), |acc, (amount, bag_type)|{
                            acc.and_then(|acc| {
                                amount_of_nested_bags.get(bag_type.as_str()).unwrap().and_then(|num|{
                                    Some(acc + num * amount)
                                })
                            })
                        })
                    },
                    None => {
                        Some(0u64)
                    }
                };

                *amount_of_nested_bags.get_mut(k.as_str()).unwrap() = amount_nested.map(|v| v + 1); // add 1 for the bag itself
            }
        });
    }

    amount_of_nested_bags.get("shiny gold").unwrap().unwrap() - 1 //exclude the shiny gold bag
}

#[cfg(test)]
mod tests {

}