use std::{
    fmt::{self, Formatter, Write},
    usize,
};

use fraction::Fraction;

pub struct Matrix {
    elements: Vec<Vec<Fraction>>,
    checksum: Fraction,
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.elements))
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.height() == 0 {
            return Ok(());
        }

        for (i, row) in self.elements.iter().enumerate() {
            f.write_fmt(format_args!("({}) [", i))?;
            for (j, n) in row.iter().enumerate() {
                f.write_fmt(format_args!("{}", n))?;
                if j < row.len() - 1 {
                    f.write_str(", ")?;
                }
            }
            f.write_char(']')?;
            if i < self.height() - 1 {
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
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            checksum: Fraction::from(0),
        }
    }

    pub fn from_iter<I, Inner>(iter: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = Inner>,
        Inner: IntoIterator<Item = Fraction>,
    {
        let iter = iter.into_iter();

        let mut matrix = Self {
            elements: Vec::with_capacity(iter.size_hint().0),
            checksum: Fraction::from(0),
        };

        for row in iter {
            matrix.insert_row(row)?;
        }

        Ok(matrix)
    }

    pub fn insert_row<I>(&mut self, row: I) -> Result<(), String>
    where
        I: IntoIterator<Item = Fraction>,
    {
        let row = row.into_iter().enumerate();
        let mut m_row = Vec::with_capacity(row.size_hint().0);
        for (i, n) in row {
            self.checksum += calc_checksum(
                &Fraction::from(self.height() as f64),
                &Fraction::from(i as f64),
                &n,
            );
            m_row.push(n);
        }

        match self.width() {
            Some(width) if width == m_row.len() => self.elements.push(m_row),
            None => self.elements.push(m_row),
            Some(width) => {
                return Err(format!(
                    "Invalid row length. Expected: {}, Got: {} when inserting: {:?}",
                    width,
                    m_row.len(),
                    m_row,
                ));
            }
        }

        Ok(())
    }

    pub fn check_xy(&self, (x, y): (usize, usize)) -> Result<(), String> {
        if x >= self.height() {
            return Err(format!(
                "Invalid row. Matrix max row index is {}, Got: {}.",
                self.height() - 1,
                x
            ));
        }

        if let Some(width) = self.width() {
            if y >= width {
                return Err(format!(
                    "Invalid column. Matrix max column index is {}, Got: {}.",
                    width - 1,
                    y
                ));
            }
        } else {
            return Err(format!(
                "Row with index {} is empty. Got column index: {}",
                x, y
            ));
        }

        Ok(())
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: Fraction) -> Result<(), String> {
        self.check_xy((x, y))?;

        let n = self.elements[x][y];
        let diff = calc_checksum(&Fraction::from(x as u64), &Fraction::from(y as u64), &n)
            - calc_checksum(&Fraction::from(x as f64), &Fraction::from(y as f64), &value);

        self.elements[x][y] = value;
        self.checksum += diff;

        Ok(())
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Result<Fraction, String> {
        self.check_xy((x, y))?;
        return Ok(self.elements[x][y]);
    }

    pub fn checksum(&self) -> Fraction {
        self.checksum
    }

    pub fn swap_rows(&mut self, lhs: usize, rhs: usize) -> Result<(), String> {
        if lhs >= self.height() {
            return Err(format!(
                "Invalid row. Matrix max row index is {}, Got: {}.",
                self.height() - 1,
                lhs
            ));
        }
        if rhs >= self.height() {
            return Err(format!(
                "Invalid row. Matrix max row index is {}, Got: {}.",
                self.height() - 1,
                rhs
            ));
        }

        self.elements.swap(lhs, rhs);

        for i in 0..self.elements[lhs].len() {
            let old = self.elements[rhs][i];
            let new = self.elements[lhs][i];

            let diff = calc_checksum(&Fraction::from(rhs as u64), &Fraction::from(i as u64), &old)
                - calc_checksum(&Fraction::from(lhs as f64), &Fraction::from(i as f64), &new);
            self.checksum += diff;
        }

        for i in 0..self.elements[rhs].len() {
            let old = self.elements[lhs][i];
            let new = self.elements[rhs][i];

            let diff = calc_checksum(&Fraction::from(lhs as u64), &Fraction::from(i as u64), &old)
                - calc_checksum(&Fraction::from(rhs as f64), &Fraction::from(i as f64), &new);
            self.checksum += diff;
        }

        Ok(())
    }

    pub fn multiply_row(&mut self, row: usize, scaler: Fraction) -> Result<(), String> {
        let len = self.elements[row].len();
        for i in 0..len {
            let xy = (row, i);
            self.set(xy, self.get(xy)? * scaler)?;
        }

        Ok(())
    }

    pub fn replace_row_with_multiple(
        &mut self,
        scaler: Fraction,
        scaler_row: usize,
        target_row: usize,
    ) -> Result<(), String> {
        let scaler_row = self.elements[scaler_row]
            .iter()
            .map(|n| n * scaler)
            .collect::<Vec<_>>();

        let len = self.elements[target_row].len();
        for i in 0..len {
            let xy = (target_row, i);
            self.set(xy, self.get(xy)? + scaler_row[i])?;
        }

        Ok(())
    }

    /// If len == 0; returns `None`
    pub fn width(&self) -> Option<usize> {
        Some(self.elements.get(0)?.len())
    }

    pub fn height(&self) -> usize {
        self.elements.len()
    }
}

fn calc_checksum(x: &Fraction, y: &Fraction, n: &Fraction) -> Fraction {
    (x + y) * *n
}
