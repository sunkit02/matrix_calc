use std::fmt::{self, Formatter};

pub enum Operations {
    SwapRows {
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
    ShowHelp,
}

impl TryFrom<&str> for Operations {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (op, rest) = match value.split_once(' ') {
            Some(value) => value,
            None => {
                let value_lower = value.to_lowercase();
                if value_lower.as_str() == "h" || value_lower.as_str() == "help" {
                    (value, "")
                } else {
                    return Err(format!(
                        "\"{}\" is not a complete instruction.",
                        value_lower
                    ));
                }
            }
        };

        match op.to_lowercase().as_str() {
            "h" | "help" => Ok(Self::ShowHelp),
            "s" => {
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

                Ok(Self::SwapRows { lhs, rhs })
            }
            "m" => {
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
            "r" => {
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
                    to_row
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
            _ => Err(format!("\"{}\" is not a valid operation.", op)),
        }
    }
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Operations::*;
        match self {
            SwapRows { lhs, rhs } => f.write_fmt(format_args!("R{} <-> R{}", lhs, rhs)),
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
            ShowHelp => f.write_str("ShowHelp"),
        }
    }
}
