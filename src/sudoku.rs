use std::fmt;

#[derive(Clone, PartialEq)]
struct Cell {
    choices: [bool; 9], 
}

impl Cell {
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
        Cell { choices: [true, true, true, true, true, true, true, true, true] }
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

    pub fn reduce(&self) -> Self {
        let mut sudoku_updated = self.clone();
        for row in 0..9 {
            for col in 0..9 {
                match self.board[row][col].unwrap() {
                    Some(n) => {
                        for r in 0..9 {
                            if r != row {
                                sudoku_updated.board[r][col].unset_at(n);
                            }
                        }
                        for c in 0..9 {
                            if c != col {
                                sudoku_updated.board[row][c].unset_at(n);
                            }
                        }
                        let district_row = row - row % 3;
                        let district_col = col - col % 3;
                        for r in district_row..district_row+3 {
                            for c in district_col..district_col+3 {
                                if r != row && c != col {
                                    sudoku_updated.board[r][c].unset_at(n);
                                }
                            }
                        }
                    }, 
                    None => {}, 
                }
            }
        }
        sudoku_updated
    }
    
    pub fn induce(&self) -> Self {
        let mut sudoku_updated = self.clone();
        for col in 0..9 {
            let mut counts = [0; 9];
            for row in 0..9 {
                let cell = &self.board[row][col];
                for n in 0..9 {
                    if cell.can_be(n+1) { counts[n] += 1; }
                }
            }
            for n in 0..9 {
                if counts[n] == 1 {
                    for row in 0..9 {
                        let cell = &mut sudoku_updated.board[row][col];
                        if cell.can_be(n+1) { cell.identify(n+1); }
                    }
                }
            }
        }
        
        for row in 0..9 {
            let mut counts = [0; 9];
            for col in 0..9 {
                let cell = &self.board[row][col];
                for n in 0..9 {
                    if cell.can_be(n+1) { counts[n] += 1; }
                }
            }
            for n in 0..9 {
                if counts[n] == 1 {
                    for col in 0..9 {
                        let cell = &mut sudoku_updated.board[row][col];
                        if cell.can_be(n+1) { cell.identify(n+1); }
                    }
                }
            }
        }
        
        // println!("{}", sudoku_updated);
        for district_row in (0..9).step_by(3) {
            for district_col in (0..9).step_by(3) {
                // println!("({}, {})", district_col, district_row);
                let mut counts = [0; 9];
                for r in district_row..district_row+3 {
                    for c in district_col..district_col+3 {
                        let cell = &self.board[r][c];
                        for n in 0..9 {
                            if cell.can_be(n+1) { counts[n] += 1; }
                        }
                    }
                }
                // println!("({}, {}): {}", district_col, district_row, counts.iter().map(|x| x.to_string()+",").collect::<String>());
                for n in 0..9 {
                    if counts[n] == 1 {
                        for r in district_row..district_row+3 {
                            for c in district_col..district_col+3 {
                                let cell = &mut sudoku_updated.board[r][c];
                                if cell.can_be(n+1) { cell.identify(n+1); }
                            }
                        }
                    }
                }
            }
        }
        sudoku_updated
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