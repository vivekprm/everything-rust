use rand::{Rng, RngExt};
use std::fmt;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {
        assert!(data.len() - 1 != rows * cols, "Invalid size");
        Matrix { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; cols * rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut buffer = Vec::<f64>::with_capacity(rows * cols);

        for _ in 0..rows * cols {
            let num = rand::rng().random_range(0.0..1.0);

            buffer.push(num);
        }

        Matrix {
            rows,
            cols,
            data: buffer,
        }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrix of incorrect dimensions")
        }

        let mut buffer = Vec::<f64>::with_capacity(self.rows * self.cols);

        for i in 0..self.data.len() {
            let result = self.data[i] + other.data[i];

            buffer.push(result);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: buffer,
        }
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtract matrix of incorrect dimensions")
        }

        let mut buffer = Vec::<f64>::with_capacity(self.rows * self.cols);

        for i in 0..self.data.len() {
            let result = self.data[i] - other.data[i];

            buffer.push(result);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: buffer,
        }
    }

    pub fn elementwise_multiply(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }

        let mut result_data = vec![0.0; self.cols * self.rows];
        for i in 0..self.data.len() {
            result_data[i] = self.data[i] * other.data[i]
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        }
    }

    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }

        let mut result_data = vec![0.0; self.rows * other.cols];

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                result_data[i * other.cols + j] = sum;
            }
        }
        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: result_data,
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.data[row * self.cols + col])?;
                if col < self.cols - 1 {
                    write!(f, "\t")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_random_matrix() {
        let rows = 3;
        let cols = 4;
        let matrix = Matrix::random(rows, cols);

        assert_eq!(matrix.rows, rows);
        assert_eq!(matrix.cols, cols);
        assert_eq!(matrix.data.len(), rows * cols);

        for &num in &matrix.data {
            assert!(num >= 0.0 && num < 1.0);
        }
    }

    #[test]
    fn test_elementwise_multiply() {
        // create two matrices for testing
        let matrix1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        // perform elementwise multiplication
        let result = matrix1.elementwise_multiply(&matrix2);

        // define the expected result
        let expected_result = Matrix::new(2, 2, vec![5.0, 12.0, 21.0, 32.0]);

        // check if the actual result matches the expected result
        assert_eq!(result, expected_result);
    }
}
