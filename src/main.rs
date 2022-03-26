mod sudoku;

fn main() {
    let problem = [
    [   [[0, 0, 6], [1, 0, 0], [0, 0, 0]], 
        [[5, 0, 0], [0, 4, 0], [0, 0, 0]], 
        [[0, 2, 0], [0, 0, 9], [7, 0, 0]]], 

    [   [[8, 0, 5], [0, 0, 6], [0, 9, 0]], 
        [[6, 0, 0], [0, 0, 1], [0, 0, 0]], 
        [[0, 0, 4], [9, 0, 2], [0, 0, 0]]], 

    [   [[0, 0, 1], [0, 0, 0], [0, 0, 0]], 
        [[0, 0, 0], [8, 0, 0], [2, 7, 0]], 
        [[0, 0, 0], [5, 0, 0], [6, 3, 0]]]]; 
    // let problem = [
    // [   [[0, 3, 0], [0, 2, 4], [0, 0, 0]], 
    //     [[0, 0, 1], [0, 3, 0], [7, 0, 5]], 
    //     [[0, 0, 0], [5, 0, 0], [8, 0, 0]]], 

    // [   [[0, 7, 0], [0, 0, 0], [0, 0, 0]], 
    //     [[0, 0, 4], [0, 8, 0], [0, 5, 7]], 
    //     [[0, 1, 0], [7, 0, 0], [9, 0, 0]]], 

    // [   [[0, 0, 6], [0, 4, 0], [3, 0, 0]], 
    //     [[0, 5, 0], [0, 0, 0], [0, 2, 0]], 
    //     [[3, 0, 7], [0, 0, 0], [0, 6, 0]]]]; 

    let mut sudoku = sudoku::Sudoku::new(problem);
    println!("{}", sudoku);
    
    // let sudoku_reduced = sudoku.reduce();
    // println!("{}", sudoku_reduced);
    
    // let sudoku_induced = sudoku_reduced.induce();
    // println!("{}", sudoku_induced);
    
    let mut sudoku_updated = sudoku.clone();
    loop {
        // println!("Reduction: ");
        let sudoku_reduced = sudoku_updated.reduce();
        // println!("{}", sudoku_reduced);
        // println!("Induction: ");
        let sudoku_induced = sudoku_reduced.induce();
        // println!("{}", sudoku_induced);
        if sudoku_updated == sudoku_induced { break; }
        sudoku_updated = sudoku_induced;
    }
    println!("{}", sudoku_updated);
}

