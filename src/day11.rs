use array2d::Array2D;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CellState
{
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let c = match self {
            CellState::Floor => '.',
            CellState::EmptySeat => 'L',
            CellState::OccupiedSeat => '#',
        };
        
        write!(f, "{}", c)
    }
}

struct CellArray<'a>(&'a Array2D<CellState>);

impl fmt::Display for CellArray<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.rows_iter().fold(Ok(()), |result, row|{
            row.fold(result, |result, s|{
                result.and_then(|_| write!(f, "{}", s))
            }).and_then(|_| writeln!(f, ""))
        })
    }
}


#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Array2D<CellState> {
    
    let rows : Vec<Vec<CellState>>= input.lines().map(|s|{
        s.chars().map(|c|{
            match c {
                '.' => CellState::Floor,
                'L' => CellState::EmptySeat,
                _ => panic!("Invalid char")
            }
        }).collect::<Vec<CellState>>()
    }).collect();

    Array2D::from_rows(&rows)
}

fn apply_rule (grid: &mut Array2D<CellState>, reference: &Array2D<CellState>, row_idx: usize, col_idx : usize) -> bool {
    const DIRECTIONS : [(isize, isize); 8]= [(-1,-1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    let count_occupied : u64 = DIRECTIONS.iter().map(|(row_delta, col_delta)|{        
        if row_idx as isize + row_delta >= 0 && col_idx as isize + col_delta >= 0 {
            if Some(&CellState::OccupiedSeat) == reference.get((row_idx as isize + *row_delta) as usize, (col_idx as isize + *col_delta) as usize) {
                return 1
            }
        }
        0
    }).sum();


    if let Some(cell) = grid.get_mut(row_idx, col_idx) {
        match cell {

            CellState::EmptySeat => { 
                if count_occupied == 0 {
                    *cell = CellState::OccupiedSeat; 
                    return true;
                }
            },
            CellState::OccupiedSeat => { 
                if count_occupied >= 4 {
                    *cell = CellState::EmptySeat; 
                    return true;
                }
            },
            _ => (),
        }
    }

    false
}

fn apply_rule_part2 (grid: &mut Array2D<CellState>, reference: &Array2D<CellState>, row_idx: usize, col_idx : usize) -> bool {
    const DIRECTIONS : [(isize, isize); 8]= [(-1,-1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    let count_occupied : u64 = DIRECTIONS.iter().map(|(row_delta, col_delta)|{        
        if row_idx as isize + row_delta < 0 || col_idx as isize + col_delta < 0 {
            return 0;
        }
        
        let mut magnitude = 1;

        loop {
            match reference.get((row_idx as isize + *row_delta * magnitude) as usize, (col_idx as isize + *col_delta * magnitude) as usize) {
                Some(&CellState::EmptySeat) => return 0,
                Some(&CellState::Floor) => magnitude += 1,
                Some(&CellState::OccupiedSeat) => return 1,
                _ => return 0
            }
        }
    }).sum();


    if let Some(cell) = grid.get_mut(row_idx, col_idx) {
        match cell {

            CellState::EmptySeat => { 
                if count_occupied == 0 {
                    *cell = CellState::OccupiedSeat; 
                    return true;
                }
            },
            CellState::OccupiedSeat => { 
                if count_occupied >= 5 {
                    *cell = CellState::EmptySeat; 
                    return true;
                }
            },
            _ => (),
        }
    }

    false
}


fn update_grid(grid: &mut Array2D<CellState>, rule : fn(&mut Array2D<CellState>, &Array2D<CellState>, usize, usize)-> bool ) -> bool {

    let const_grid = grid.clone();
    let mut any_updates = false;
    const_grid.rows_iter().enumerate().for_each(|(row_idx, row)|{
        row.enumerate().for_each(|(col_idx, _)|{
            any_updates |= rule(grid, &const_grid, row_idx, col_idx);
        });
    });

    any_updates
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Array2D<CellState>) -> u64 {
    let mut main_grid = input.clone();

    loop {
        let updated = update_grid(&mut main_grid, apply_rule);

        if !updated {
            break;
        }
    }

    main_grid.elements_row_major_iter().map(|state|{
        if *state == CellState::OccupiedSeat {
            1
        }
        else {
            0
        }
    }).sum()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Array2D<CellState>) -> u64 {
    let mut main_grid = input.clone();

    loop {
        let updated = update_grid(&mut main_grid, apply_rule_part2);

        if !updated {
            break;
        }
    }

    main_grid.elements_row_major_iter().map(|state|{
        if *state == CellState::OccupiedSeat {
            1
        }
        else {
            0
        }
    }).sum()
}


#[cfg(test)]
mod tests {
 
}