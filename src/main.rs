use std::io::{stdin, stdout, Write};
use std::str::FromStr;

use fraction::{Fraction, ToPrimitive};

use crate::{matrix::Matrix, operations::Operations};

mod matrix;
mod operations;

// TODO: Refactor into lib and bin crates
fn main() {
    'outer: loop {
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

            for token in line.trim().split(' ') {
                match Fraction::from_str(token) {
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

        println!(
            "\nMatrix (checksum: {}):",
            matrix
                .checksum()
                .to_f64()
                .expect("Failed to convert from `Fraction` to `f64")
        );
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

            println!("$ {}\n", op);
            match op {
                Operations::ShowHelp => print_help_menu(),
                Operations::SwapRows { lhs, rhs } => match matrix.swap_rows(lhs, rhs) {
                    Ok(_) => {
                        println!("Matrix (checksum: {}):", matrix.checksum());
                        println!("\n{}\n", matrix);
                    }
                    Err(e) => println!("Error: {}", e),
                },
                Operations::Multiply { row, scaler } => match matrix.multiply_row(row, scaler) {
                    Ok(_) => {
                        println!("Matrix (checksum: {}):", matrix.checksum());
                        println!("\n{}\n", matrix);
                    }
                    Err(e) => println!("Error: {}", e),
                },
                Operations::ReplaceWithMultiple {
                    scaler,
                    scaler_row,
                    target_row,
                } => match matrix.replace_row_with_multiple(scaler, scaler_row, target_row) {
                    Ok(_) => {
                        println!("Matrix (checksum: {}):", matrix.checksum());
                        println!("\n{}\n", matrix);
                    }
                    Err(e) => println!("Error: {}", e),
                },
                Operations::ClearScreen => clear_screen(),
                Operations::ShowMatrix => println!("{}\n", matrix),
                Operations::Restart => {
                    clear_screen();
                    continue 'outer;
                }
                Operations::ExitProgram => {
                    println!("\nExiting program...");
                    std::process::exit(0);
                }
            }

            print!("> ");
            stdout().flush().expect("Failed to flush stdout.");
        }
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

VALID COMMANDS:
    Clear screen                    c or clear
    Show matrix                     show
    Show help                       h or help
    Restart                         restart
    Exit program                    q or exit
"#
    );
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
