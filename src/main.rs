use std::io::{stdin, stdout, Write};

use crate::{matrix::Matrix, operations::Operations};

mod matrix;
mod operations;

fn main() {
    println!(
        "Please enter values of each row for your matrix space separeted (Empty row to stop):"
    );
    print!("> ");
    stdout().flush().expect("Failed to flush stdout.");

    let mut matrix = Matrix::new();

    let mut row = Vec::new();
    'read: while let Some(Ok(line)) = stdin().lines().next() {
        row.clear();

        if line.is_empty() {
            println!("\nEnd of row entry.\n");
            break 'read;
        }

        for c in line.trim().split(' ') {
            match c.parse::<f64>() {
                Ok(n) => row.push(n),
                Err(e) => {
                    println!("Error: {}.", e);
                    println!("Please ensure that the numbers are separated by only one space.");

                    print!("> ");
                    stdout().flush().expect("Failed to flush stdout.");
                    continue 'read;
                }
            }
        }

        if let Err(e) = matrix.insert_row(row.clone()) {
            println!("Error: {}.", e);
        }

        print!("> ");
        stdout().flush().expect("Failed to flush stdout.");
    }

    if matrix.height() == 0 {
        println!("You have an empty matrix, exiting...");
        return;
    }

    println!("\nMatrix (checksum: {}):", matrix.checksum());
    println!("{}\n", matrix);
    print!("> ");
    stdout().flush().expect("Failed to flush stdout.");

    while let Some(Ok(line)) = stdin().lines().next() {
        if line.is_empty() {
            print!("> ");
            stdout().flush().expect("Failed to flush stdout.");
            continue;
        }

        let op = match Operations::try_from(line.as_str()) {
            Ok(op) => op,
            Err(e) => {
                println!("Error: {}", e);

                print!("> ");
                stdout().flush().expect("Failed to flush stdout.");
                continue;
            }
        };

        println!("$ {}", op);
        match op {
            Operations::ShowHelp => print_help_menu(),
            op => match matrix.operate(op) {
                Ok(_) => {
                    println!("Matrix (checksum: {}):", matrix.checksum());
                    println!("\n{}\n", matrix);
                }
                Err(e) => println!("Error: {}", e),
            },
        }

        print!("> ");
        stdout().flush().expect("Failed to flush stdout.");
    }
}

fn print_help_menu() {
    println!(
        r#"
VALID OPERATIONS:
    Operation                       Syntax

    Swap two rows                   S (row1 index) (row2 index)

    Multiple a row                  M (scaler value) (row index)

    Replace `target row` with       R (scaler value) (scaler row index) (target row index)
    the product of the `scaler 
    row` with `scaler value`
"#
    );
}
