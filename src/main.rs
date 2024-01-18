use std::fmt::{self, Formatter};
use std::io::Write;

struct Matrix {
    elements: Vec<Vec<f64>>,
    checksum: f64,
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.elements))
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let last = self.elements.len() - 1;
        for (i, row) in self.elements.iter().enumerate() {
            f.write_fmt(format_args!("{:?}", row))?;
            if i != last {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        if self.checksum != rhs.checksum {
            println!("Different checksums!");
            return false;
        }

        return self.elements == rhs.elements;
    }
}

impl Matrix {
    fn from_iter<I, Inner>(iter: I) -> Self
    where
        I: IntoIterator<Item = Inner>,
        Inner: IntoIterator<Item = f64>,
    {
        let matrix = iter.into_iter().enumerate();

        let mut checksum: f64 = 0.0;
        let mut elements = Vec::with_capacity(matrix.size_hint().0);

        for (i, row) in matrix {
            let row = row.into_iter().enumerate();
            let mut m_row = Vec::with_capacity(row.size_hint().0);
            for (j, n) in row {
                checksum += Self::checksum(i as f64, j as f64, n);
                m_row.push(n);
            }
            elements.push(m_row);
        }

        Self { elements, checksum }
    }
    fn set(&mut self, (x, y): (usize, usize), value: f64) {
        let n = self.elements[x][y];
        let diff =
            Self::checksum(x as f64, y as f64, n) - Self::checksum(x as f64, y as f64, value);

        self.elements[x][y] = value;
        self.checksum += diff;
    }
    fn get(&mut self, (x, y): (usize, usize)) -> f64 {
        return self.elements[x][y];
    }

    fn checksum(x: f64, y: f64, n: f64) -> f64 {
        (x + y) * n
    }

    fn operate(&mut self, op: Operations) {
        use Operations::*;
        match op {
            ExchangeRows { lhs, rhs } => self.elements.swap(lhs, rhs),
            Multiply { row, scaler } => {
                for n in &mut self.elements[row] {
                    *n *= scaler
                }
            }
            ReplaceWithMultiple {
                scaler,
                from_row,
                to_row,
            } => {
                let scaler_row = self.elements[from_row]
                    .iter()
                    .map(|n| n * scaler)
                    .collect::<Vec<_>>();
                for (i, n) in self.elements[to_row].iter_mut().enumerate() {
                    *n += scaler_row[i];
                }
            }
        }
    }
}

enum Operations {
    ExchangeRows {
        lhs: usize,
        rhs: usize,
    },
    Multiply {
        row: usize,
        scaler: f64,
    },
    ReplaceWithMultiple {
        scaler: f64, // cannot be zero
        from_row: usize,
        to_row: usize,
    },
}

impl TryFrom<&str> for Operations {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (op, rest) = value
            .split_once(' ')
            .ok_or(format!("\"{}\" is not a valid instruction.", value))?;

        match op {
            "E" => {
                let (lhs, rhs) = if let Some(rest) = rest.split_once(' ') {
                    rest
                } else {
                    return Err(format!(
                        "Expected two space separated row indices. Got: \"{}\"",
                        rest
                    ));
                };

                let (lhs, rhs) = (
                    lhs.to_lowercase()
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>(),
                    rhs.to_lowercase()
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>(),
                );

                let (lhs, rhs) = (
                    lhs.parse::<usize>()
                        .map_err(|_| format!("Failed to parse \"{}\" to `usize`", lhs))?,
                    rhs.parse::<usize>()
                        .map_err(|_| format!("Failed to parse \"{}\" to `usize`", rhs))?,
                );

                Ok(Self::ExchangeRows { lhs, rhs })
            }
            "M" => {
                let (scaler, row) = if let Some(rest) = rest.split_once(' ') {
                    rest
                } else {
                    return Err(format!(
                        "Expected a scaler and a row index separated by a space. Got: \"{}\"",
                        rest
                    ));
                };

                let scaler = scaler
                    .parse::<f64>()
                    .map_err(|_| format!("Failed to parse \"{}\".", scaler))?;

                let row = row
                    .to_lowercase()
                    .chars()
                    .filter(|c| *c != 'r')
                    .collect::<String>()
                    .parse::<usize>()
                    .map_err(|_| format!("Failed to parse \"{}\".", row))?;

                Ok(Self::Multiply { row, scaler })
            }
            "R" => {
                let (scaler, rows) = if let Some(rest) = rest.split_once(' ') {
                    rest
                } else {
                    return Err(format!(
                        "Expected a scaler and two row indices separated by spaces. Got: \"{}\"",
                        rest
                    ));
                };

                let (from_row, to_row) = if let Some(rows) = rows.split_once(' ') {
                    rows
                } else {
                    return Err(format!(
                        "Expected two row indices separated by a space. Got: \"{}\"",
                        rows
                    ));
                };

                let scaler = scaler
                    .parse::<f64>()
                    .map_err(|_| format!("Failed to parse \"{}\".", scaler))?;

                let (from_row, to_row) = (
                    from_row
                        .to_lowercase()
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>()
                        .parse::<usize>()
                        .map_err(|_| format!("Failed to parse \"{}\".", from_row))?,
                    from_row
                        .to_lowercase()
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>()
                        .parse::<usize>()
                        .map_err(|_| format!("Failed to parse \"{}\".", to_row))?,
                );

                Ok(Self::ReplaceWithMultiple {
                    scaler,
                    from_row,
                    to_row,
                })
            }
            _ => Err(format!("{} is not a valid operation.", op)),
        }
    }
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Operations::*;
        match self {
            ExchangeRows { lhs, rhs } => f.write_fmt(format_args!("E R{} R{}", lhs, rhs)),
            Multiply { row, scaler } => {
                f.write_fmt(format_args!("{} * R{} -> R{}", scaler, row, row))
            }
            ReplaceWithMultiple {
                scaler,
                from_row,
                to_row,
            } => f.write_fmt(format_args!(
                "{} * R{} + R{} -> R{}",
                scaler, from_row, to_row, to_row
            )),
        }
    }
}

fn main() {
    let mut matrix = Matrix::from_iter([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    // let mut b = Matrix::from_iter([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);

    // let operations = [
    //     Operations::ExchangeRows { lhs: 1, rhs: 2 },
    //     Operations::Multiply {
    //         row: 0,
    //         scaler: -1.5,
    //     },
    //     Operations::ReplaceWithMultiple {
    //         scaler: 2.,
    //         from_row: 1,
    //         to_row: 0,
    //     },
    // ];
    //
    // println!("{}\n", a);
    //
    // for op in operations {
    //     println!("{}\n", op);
    //     a.operate(op);
    //     println!("{}\n", a);
    // }

    println!("Matrix:");
    println!("{}\n", matrix);
    print!("> ");
    std::io::stdout().flush().unwrap();

    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        let op = match Operations::try_from(line.as_str()) {
            Ok(op) => op,
            Err(err) => {
                println!("Error: {}", err);

                print!("> ");
                std::io::stdout().flush().unwrap();
                continue;
            }
        };

        println!("$ {}", op);
        matrix.operate(op);
        println!("Matrix:");
        println!("\n{}\n", matrix);
        print!("> ");
        std::io::stdout().flush().unwrap();
    }
}
