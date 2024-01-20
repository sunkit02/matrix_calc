use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

use fraction::Fraction;

pub enum Operations {
    SwapRows {
        lhs: usize,
        rhs: usize,
    },
    Multiply {
        row: usize,
        scaler: Fraction,
    },
    ReplaceWithMultiple {
        scaler: Fraction, // cannot be zero
        scaler_row: usize,
        target_row: usize,
    },
    ShowHelp,
    // TODO: SetValue
    // TODO: ShowMatrix
    // TODO: Undo
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

                let scaler = Fraction::from_str(scaler)
                    .map_err(|e| format!("Failed to parse \"{}\". {}", scaler, e))?;

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

                let (scaler_row, target_row) = if let Some(rows) = rows.split_once(' ') {
                    rows
                } else {
                    return Err(format!(
                        "Expected two row indices separated by a space. Got: \"{}\"",
                        rows
                    ));
                };

                let scaler = Fraction::from_str(scaler)
                    .map_err(|e| format!("Failed to parse \"{}\". {}", scaler, e))?;

                let (scaler_row, target_row) = (
                    scaler_row
                        .to_lowercase()
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>()
                        .parse::<usize>()
                        .map_err(|_| format!("Failed to parse \"{}\".", scaler_row))?,
                    target_row
                        .to_lowercase()
                        .chars()
                        .filter(|c| *c != 'r')
                        .collect::<String>()
                        .parse::<usize>()
                        .map_err(|_| format!("Failed to parse \"{}\".", target_row))?,
                );

                Ok(Self::ReplaceWithMultiple {
                    scaler,
                    scaler_row,
                    target_row,
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
                scaler_row,
                target_row,
            } => f.write_fmt(format_args!(
                "{} * R{} + R{} -> R{}",
                scaler, scaler_row, target_row, target_row
            )),
            ShowHelp => f.write_str("ShowHelp"),
        }
    }
}
