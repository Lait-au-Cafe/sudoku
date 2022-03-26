use std::fmt;
use itertools::Itertools;
use std::convert::TryInto;

#[derive(Clone, PartialEq)]
struct Cell {
    choices: [bool; 9], 
}

impl Cell {
    // ...
    pub fn from_choices_list(choices: [bool; 9]) -> Self {
        Cell { choices: choices }
    }

    // ...
    pub fn unwrap(&self) -> Option<usize> {
        let mut choices = self.choices.iter();
        let first_candidate = choices.position(|x| *x).expect("Fatal Error: No candidate is found at this cell. ");
        match choices.position(|x| *x) {
            Some(_) => { None }, 
            None => { Some(first_candidate+1) }, 
        }
    }

    // ...
    pub fn can_be(&self, n: usize) -> bool { self.choices[n-1] }

    // ...
    pub fn identify(&mut self, n: usize) {
        for i in 1..=9 { self.unset_at(i); }
        self.set_at(n);
    }

    // ...
    pub fn set_at(&mut self, n: usize) { self.choices[n-1] = true; }
    // ...
    pub fn unset_at(&mut self, n: usize) { self.choices[n-1] = false; }

    // ...
    pub fn size(&self) -> usize {
        self.choices.iter().fold(0, |acc, x| if *x {acc+1} else {acc})
    }

    // ...
    pub fn difference(&self, other: &Self) -> Self {
        let mut diff = self.clone();
        for n in 1..=9 {
            if other.can_be(n) {
                diff.unset_at(n);
            }
        }
        diff
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell { choices: [true; 9] }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct Sudoku {
    board: [[Cell; 9]; 9], 
}

impl Sudoku {
    pub fn new(problem: [[[[usize; 3]; 3]; 3]; 3]) -> Self {
        let mut board: [[Cell; 9]; 9] = Default::default();

        // ...
        for outer_y in 0..3 {
            for outer_x in 0..3 {
                for inner_y in 0..3 {
                    for inner_x in 0..3 {
                        match problem[outer_y][inner_y][outer_x][inner_x] {
                            n @ 1..=9 => {
                                let row = outer_y * 3 + inner_y;
                                let col = outer_x * 3 + inner_x;
                                board[row][col].identify(n);
                            },
                            _ => {}, 
                        }
                    }
                }
            }
        }

        Sudoku { board: board }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut diff = self.clone();
        for row in 0..9 {
            for col in 0..9 {
                diff.board[row][col] = self.board[row][col].difference(&other.board[row][col]);
            }
        }
        diff
    }

    pub fn reduce(&self, order: usize) -> Self {
        let mut sudoku_updated = self.clone();

        for row in 0..9 {
            let coords = (0..9).map(|col| (col, row));
            let coords: [(usize, usize); 9] = coords.collect::<Vec<_>>().try_into().unwrap();
            self.reduce_core(&mut sudoku_updated, coords, order);
        }
        
        for col in 0..9 {
            let coords = (0..9).map(|row| (col, row));
            let coords: [(usize, usize); 9] = coords.collect::<Vec<_>>().try_into().unwrap();
            self.reduce_core(&mut sudoku_updated, coords, order);
        }
        
        for district_row in (0..9).step_by(3) {
            for district_col in (0..9).step_by(3) {
                let coords = (district_col..district_col+3).cartesian_product(district_row..district_row+3);
                let coords: [(usize, usize); 9] = coords.collect::<Vec<_>>().try_into().unwrap();
                self.reduce_core(&mut sudoku_updated, coords, order);
            }
        }

        sudoku_updated
    }

    fn reduce_core(&self, dest: &mut Sudoku, coords: [(usize, usize); 9], k: usize) {
        for xs in coords.iter().filter(|x| self.board[x.1][x.0].size() <= k).combinations(k) {
            // ...
            let mut compound_cell = Cell::from_choices_list([false; 9]);
            for (x, y) in xs { for n in 0..9 { if self.board[*y][*x].can_be(n+1) { compound_cell.set_at(n+1) } }}

            // ...
            let cnt = compound_cell.size();
            if cnt == k {
                for (x, y) in coords {
                    if dest.board[y][x].difference(&compound_cell).size() > 0 {
                        for n in 0..9 { if compound_cell.can_be(n+1) { dest.board[y][x].unset_at(n+1); }}
                    }
                }
            }
            else if cnt < k { panic!("Fatal Error: Candidates are less than cells. "); }
        }

    }
}


impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gridline_row = ("-".repeat(4*3+1) + " ").repeat(3);
        for row in 0..9 {
            if row % 3 == 0 { write!(f, "{}\n", gridline_row)?; }
            for k in 0..3 {
                for col in 0..9 {
                    if col % 3 == 0 { write!(f, "|")?; }
                    for n in 3*k+1..3*(k+1)+1 {
                        let c = if self.board[row][col].can_be(n) { n.to_string() } else { " ".to_string() };
                        write!(f, "{}", c)?;
                    }
                    write!(f, "|")?;
                    if col % 3 == 2 { write!(f, " ")?; }
                }
                write!(f, "\n")?;
            }
            write!(f, "{}\n", gridline_row)?;
        }
        fmt::Result::Ok(())
    }
}