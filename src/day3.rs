#[derive(PartialEq)]
pub enum MapTile
{
    Tree,
    Open
}

impl From<char> for MapTile
{
    fn from(c: char) -> Self {
            match c
            {
                '#' => MapTile::Tree,
                '.' => MapTile::Open,
                _ => panic!("Unrecognized char for map.")
            }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<MapTile>> {
    input
        .lines()
        .map(|l| { 
            l.chars().map(MapTile::from).collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<MapTile>>) -> u64 {
      input
        .iter()
        .skip(1)
        .fold((0, 0), |acc, p| {
            let x = (acc.0 + 3) % p.len();

            if p[x] == MapTile::Tree { 
                (x, acc.1 + 1)
            } 
            else { 
                (x, acc.1) 
            }
        }).1
}


fn slope_calc (input: &Vec<Vec<MapTile>>, right: usize, down: usize) -> u64
{
      input
        .iter()
        .skip(down)
        .step_by(down)
        .fold((0, 0), |acc, p| {
            let x = (acc.0 + right) % p.len();

            if p[x] == MapTile::Tree { 
                (x, acc.1 + 1)
            } 
            else { 
                (x, acc.1) 
            }
        }).1
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<MapTile>>) -> u64 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|slope| slope_calc(input,slope.0, slope.1))
        .product()
}

#[cfg(test)]
mod tests {

}