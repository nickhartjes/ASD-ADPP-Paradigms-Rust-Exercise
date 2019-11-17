use crate::MatrixActions::{ADD, REMOVE};

enum MatrixActions {
    ADD,
    REMOVE,
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let return_value: Option<Vec<(u8, u8)>> = Some(vec![]);

    // If the input is empty, return directly.
    if input.len() <= 0 {
        return return_value;
    }

    print_matrix(create_matrix(input));

    for domino in input {
        search_longest_chain(domino, create_matrix(input));
    }

    return return_value;
}

pub fn print_matrix(matrix: Vec<Vec<u8>>) {
    // Prints the given matrix in a 2d form
    // Columns an row numbers are shown
    let mut row_count: u8 = 0;
    for row in matrix {
        if row_count == 0 {
            print!("  ");
            for x in 0..row.len() {
                print!(" {} ", x);
            }
            print!("\n");
        }
        print!("{} ", row_count);
        println!("{:?}", row);
        row_count = row_count + 1;
    }
}

pub fn create_matrix(input: &[(u8, u8)]) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; 7]; 7];

    // Loop the slice input, that contains tuples of u8
    for domino in input {
        matrix = update_matrix(domino, matrix, ADD);
    }
    return matrix;
}


pub fn search_longest_chain(domino: &(u8, u8), matrix: Vec<Vec<u8>>) -> u16 {
    let mut tmp_matrix = update_matrix(domino, matrix, REMOVE);

    check_matching_side(domino, tmp_matrix);

    println!("-- {:?}", domino);

    return 0;
}

fn update_matrix(domino: &(u8, u8), matrix: Vec<Vec<u8>>, action: MatrixActions) -> Vec<Vec<u8>> {
    let mut tmp_matrix = matrix.clone();
    let row_location = domino.0 as usize;
    let column_location = domino.1 as usize;
    match action {
        ADD => tmp_matrix[row_location][column_location] = 1,
        REMOVE => tmp_matrix[row_location][column_location] = 0
    };
    return tmp_matrix;
}

fn check_matching_side(domino: &(u8, u8), matrix: Vec<Vec<u8>>) -> &[(u8, u8)] {
    let mut found: &[(u8, u8)] = &[];
    let row_number = domino.0 as usize;
    let column_number = domino.1 as usize;

    let mut iterator = matrix.into_iter();
    let mut counter = 0;
    loop {
        match iterator.next() {
            Some(number) => {
                if number[column_number] == 1 {
                    println!("*** {}", number[column_number]);
                    println!("*** {}", counter);
                }
                counter = counter + 1;
            },
            None => break,
        }
    }

//    for value in matrix[0].into_iter() {
//        match value {
//            num @ 0 => println!("Lower Range: {}", num),
//            num @ 1 => println!("Upper Range: {}", num),
//            _ => println!("Not in range."),
//        }
//    }

//    let mut iterator = matrix.into_iter();
//    loop {
//        match iterator.next() {
//            Some(number) => print!("{}", number),
//            None => break,
//        }
//    }
    return found;
}