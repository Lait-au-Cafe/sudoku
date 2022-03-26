mod sudoku;

fn main() {
    let problem = [
    [   [[0, 0, 0], [3, 4, 0], [1, 0, 0]], 
        [[0, 0, 0], [0, 0, 0], [0, 3, 0]], 
        [[0, 0, 8], [0, 7, 0], [0, 4, 0]]], 

    [   [[0, 1, 0], [0, 0, 0], [0, 0, 9]], 
        [[0, 8, 0], [0, 0, 0], [0, 6, 1]], 
        [[3, 4, 7], [0, 0, 0], [0, 0, 0]]], 

    [   [[0, 0, 2], [1, 0, 0], [0, 0, 0]], 
        [[0, 0, 0], [0, 0, 6], [2, 0, 5]], 
        [[7, 0, 0], [0, 0, 8], [0, 0, 0]]]]; 

    let sudoku = sudoku::Sudoku::new(problem);
    println!("{}", sudoku);
    
    let mut sudoku_updated = sudoku.clone();
    loop {
        let mut sudoku_reduced = sudoku_updated.clone();
        for i in 1..=8 {
            sudoku_reduced = sudoku_reduced.reduce(i);
        }
        if sudoku_updated == sudoku_reduced { break; }
        sudoku_updated = sudoku_reduced;
    }
    println!("{}", sudoku_updated);
}

