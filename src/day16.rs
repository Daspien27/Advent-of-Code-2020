use std::collections::{BTreeMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Range
{
    lower: u64,
    upper: u64,
}

impl Range {
    fn contains(&self, v: &u64) -> bool{
        self.lower <= *v && *v <= self.upper
    }
}

type Ticket = Vec<u64>;

#[derive(Debug)]
pub struct TicketTranslator {
    props : BTreeMap<String, Vec<Range>>,
    main_ticket : Ticket,
    nearby_tickets: Vec<Ticket>
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> TicketTranslator {
    lazy_static! {
        static ref RANGE_RE : Regex = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)").unwrap();
        static ref PROP_RE : Regex = Regex::new(r"(?P<prop>[\w ]+):(?P<ranges>(:?(:? \d+-\d+)(:? or)?)+)").unwrap();
        static ref TICKET_RE : Regex = Regex::new(r"(?m)^(\d+,?)+").unwrap();

    }
    
    let props = PROP_RE.captures_iter(input).map(|cap|{
        let prop = String::from(cap.name("prop").unwrap().as_str());
        let ranges : Vec<Range> = RANGE_RE.captures_iter(cap.name("ranges").unwrap().as_str()).map(|cap|{
            Range{lower: cap.name("lower").unwrap().as_str().parse().unwrap(),
                  upper: cap.name("upper").unwrap().as_str().parse().unwrap()}
        }).collect();
        (prop, ranges)
    }).collect::<BTreeMap<String, Vec<Range>>>();
    
    let tickets : Vec<Ticket> = TICKET_RE.captures_iter(input).map(|cap|{
        cap.get(0).unwrap().as_str().split(",").map(|s| s.parse().unwrap()).collect()
    }).collect();

    TicketTranslator{props: props, main_ticket: tickets.first().unwrap().clone(), nearby_tickets: tickets.iter().skip(1).cloned().collect() }
}


#[aoc(day16, part1)]
pub fn solve_part1(input: &TicketTranslator) -> u64 {

    input.nearby_tickets.iter().flatten().filter(|v|{
        !input.props.values().flatten().any(|range|{
            range.contains(*v)
        })
    }).sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &TicketTranslator) -> u128 {

    let valid_tickets : Vec<_> = input.nearby_tickets.iter().filter(|vs|{
        vs.iter().all(|v|{
                input.props.values().flatten().any(|range|{
                range.lower <= *v && *v <= range.upper
            })
        })
    }).collect::<Vec<&Ticket>>();

    assert_eq!(input.props.len(), input.main_ticket.len());
    let num_props = input.props.len();
    let mut properties = vec![(0..num_props).collect::<HashSet<usize>>(); num_props];

    valid_tickets.iter().for_each(|vs|{
        vs.iter().enumerate().for_each(|(i,v)|{
            input.props.iter().enumerate().for_each(|(k, (_, ranges))|{
                    if !ranges.iter().any(|r| r.contains(v)) {
                        properties[i].remove(&k);
                    }
            });
        });
    });

    let mut undiscovered : HashSet<usize> = (0..num_props).collect();

    while !properties.iter().all(|p| p.len() == 1) {

        let undiscovered_prop = undiscovered.iter().find(|prop|{
            properties.iter().find(|p| p.contains(prop) && p.len() == 1).is_some()
            || properties.iter().filter(|p| p.contains(prop)).count() == 1
        });

        if let Some(&prop) = undiscovered_prop {

            let number_viable_props = properties.iter().filter(|p| p.contains(&prop)).count();

            if number_viable_props > 1 {
                properties.iter_mut().for_each(|hs| {

                    if hs.len() > 1 {
                        hs.remove(&prop);
                    }
                });
            }
            undiscovered.remove(&prop);
        }
    }

    assert!(properties.iter().all(|p| p.len() == 1));
    assert!(input.main_ticket.iter().zip(properties.iter()).all(|(p, ranges)|{
        let prop = input.props.iter().skip(*ranges.iter().next().unwrap()).next().unwrap();
        prop.1.iter().any(|r| r.contains(p))
    }));

    input.main_ticket.iter().zip(properties.iter()).map(|(ticket_prop, hs)|{
        (ticket_prop, hs.iter().next().unwrap())
    }).filter(|(_, prop)|{
        lazy_static! {
            static ref DEPARTURE_RELATED_RE : Regex = Regex::new(r"departure").unwrap();
        }

        let prop_name = input.props.keys().skip(**prop).next().unwrap();

        DEPARTURE_RELATED_RE.is_match(prop_name.as_str())
    }).map(|(ticket_prop, _)| {
        *ticket_prop as u128
    }).product()
}

#[cfg(test)]
mod tests {

}