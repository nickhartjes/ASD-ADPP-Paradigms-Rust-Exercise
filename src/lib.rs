use crate::MatrixActions::{ADD, REMOVE};

enum MatrixActions {
    ADD,
    REMOVE,
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    // If the input is empty, return directly.
    if input.is_empty() {
        return Some(vec![]);
    }

    // Just a visualisation step, can be removed for performance
    print_matrix(create_matrix(input));

    // Start a search from all the dominoes, and stop if the first full path  is found
    for domino in input {
        let chain = search_chain(*domino, input.len(), create_matrix(input), Vec::new());
        if let Some(chain) = chain {
            return Some(chain);
        }
    }
    None
}

fn print_matrix(matrix: Vec<Vec<u8>>) {
    // Prints the given matrix in a 2d shape
    // Columns an row numbers are shown
    // Example:
    //   0  1  2  3  4  5  6
    //0 [0, 0, 0, 0, 0, 0, 0]
    //1 [0, 0, 1, 0, 0, 0, 0]
    //2 [0, 0, 0, 1, 0, 0, 0]
    //3 [0, 1, 0, 0, 0, 0, 0]
    //4 [0, 0, 0, 0, 0, 0, 0]
    //5 [0, 0, 0, 0, 0, 0, 0]
    //6 [0, 0, 0, 0, 0, 0, 0]
    for (row_count, row) in matrix.iter().enumerate() {
        if row_count == 0 {
            print!("  ");
            for x in 0..row.len() {
                print!(" {} ", x);
            }
            println!();
        }
        print!("{} ", row_count);
        println!("{:?}", row);
    }
}

fn create_matrix(input: &[(u8, u8)]) -> Vec<Vec<u8>> {
    // Creates a 2d Vector matrix from given domino slices
    let matrix_size: usize = 7;
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; matrix_size]; matrix_size];

    // Loop the slice input, that contains tuples of u8
    for domino in input {
        matrix = update_matrix(*domino, matrix, ADD);
    }
    matrix
}

fn search_chain(
    domino: (u8, u8),
    dominoes_chain_length: usize,
    matrix: Vec<Vec<u8>>,
    mut path: Vec<(u8, u8)>,
) -> Option<Vec<(u8, u8)>> {
    // Remove the current domino from the matrix
    let tmp_matrix = update_matrix(domino, matrix, REMOVE);
    // Create the result path
    let mut result: Vec<(u8, u8)> = Vec::new();

    // To match
    let mut first = 0;
    let mut last = u8::max_value();
    if !path.is_empty() {
        first = path[0].0;
        last = path[path.len() - 1].1
    }

    if last == domino.0 || path.is_empty() {
        path.push(domino);
        first = path[0].0;
        last = path[path.len() - 1].1
    } else {
        // insert reverse domino
        path.push((domino.1, domino.0));
        last = domino.0;
    }

    // Check correct path
    if path.len() == dominoes_chain_length && first == last {
        println!("Found path:  {:?} ", path);
        Some(path)
    } else {
        // Search both sides of the domino
        result.append(search_in_row(domino.1, tmp_matrix.clone()).as_mut());
        result.append(search_in_column(domino.1, tmp_matrix.clone()).as_mut());

        // Recursive call to the nodes found
        if !result.is_empty() {
            for x in result.iter() {
                let row = x.0;
                let column = x.1;
                return search_chain(
                    (row, column),
                    dominoes_chain_length,
                    tmp_matrix.clone(),
                    path.clone(),
                );
            }
        }
        None
    }
}

fn update_matrix(domino: (u8, u8), matrix: Vec<Vec<u8>>, action: MatrixActions) -> Vec<Vec<u8>> {
    let mut tmp_matrix = matrix.clone();
    let row_location = domino.0 as usize;
    let column_location = domino.1 as usize;
    match action {
        ADD => tmp_matrix[row_location][column_location] = 1,
        REMOVE => tmp_matrix[row_location][column_location] = 0,
    };
    tmp_matrix
}

fn search_in_column(column_number: u8, matrix: Vec<Vec<u8>>) -> Vec<(u8, u8)> {
    let mut result: Vec<(u8, u8)> = Vec::new();
    for (row_counter, row) in matrix.iter().enumerate() {
        if let 1 = row[column_number as usize] {
            let domino = (row_counter as u8, column_number);
            result.push(domino);
        }
    }
    result
}

fn search_in_row(row_number: u8, matrix: Vec<Vec<u8>>) -> Vec<(u8, u8)> {
    let mut result: Vec<(u8, u8)> = Vec::new();
    for (column_counter, value) in matrix[row_number as usize].iter().enumerate() {
        if let 1 = value {
            let domino = (row_number, column_counter as u8);
            result.push(domino);
        }
    }
    result
}
