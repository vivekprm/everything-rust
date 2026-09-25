# Neural Networks
First lets get the mental model of what Neural Network is.

Imagine a factory responsible for producing solid metal speheres. Factory orders metal cubes of a certain size and places them into
their special cutting machine which outputs solid metal spheres.

The workers place the cubes onto and assembly line and off it goes into the machine. The whole factory loves this cutting machine but
they start to become old and rusted. The factory owner calls the company and they send you the lead engineer out to fix the machines.

You first place cube into the machine to ascertain the state of the gears. The end result is half cube, half sphere shape.

<img width="777" height="497" alt="Screenshot 2026-09-25 at 10 12 57 AM" src="https://github.com/user-attachments/assets/73d70dff-6648-4389-9839-b2a5a14f0f90" />

Now the engineer realizes something, every layer of gears is dependent upon the layer before it. By tracing the gears we can see exactly how
each chain of gears affects the output of our machine.

<img width="1196" height="268" alt="Screenshot 2026-09-25 at 10 17 24 AM" src="https://github.com/user-attachments/assets/2490dbfa-4a9b-436f-aa50-3fc0b4bfb68f" />

The engineer tweaks some gears in the chain he can see that are the most responsible for the incorrect output and turns on the machine to try
with another Cube. Success!

He realizes that the new shape is closer to a sphere but he's still off. He continues to repeat the following steps:
- **Forward Propagation**: Pushing the cube through the machine
- **Cost Function**: Where he calculates how close the output is to a perfect sphere.
- **Back Propagation**: He goes back into the gears and updates them.

This machine you may have guessed by now is a Neural Network.

<img width="844" height="569" alt="Screenshot 2026-09-25 at 10 35 11 AM" src="https://github.com/user-attachments/assets/a137a0e6-0011-4d1f-89a5-581e56040882" />

Let's zoom in on just one of these circles. It's called a **Neuron**, meant to loosly mirror how the brain works.

<img width="1125" height="536" alt="Screenshot 2026-09-25 at 10 36 32 AM" src="https://github.com/user-attachments/assets/a92e8abc-8a35-4e6f-a1df-49b83732483e" />

Let's take a look at a two input Neuron.

# Neuron

<img width="847" height="352" alt="Screenshot 2026-09-25 at 10 37 32 AM" src="https://github.com/user-attachments/assets/e9137eea-2e98-48b0-8466-7bba43e9e80c" />

Three things are happening here. First each of our inputs is multiplied by it's appropriate weight.

<img width="971" height="494" alt="Screenshot 2026-09-25 at 10 38 53 AM" src="https://github.com/user-attachments/assets/78cfd039-a5bc-445a-a959-f18ccafe69aa" />

A weight is something that determines the strength or importance of each input signal.

Next all the weighted inputs are added together with a **Bias**.

<img width="786" height="565" alt="Screenshot 2026-09-25 at 10 40 32 AM" src="https://github.com/user-attachments/assets/c8ab495a-d96c-4b83-ab94-98c410367536" />

Finally, this now weighted sum is passed through an **activation function**. The reason we use an **activation function** is to make the
neuron non-linear.

<img width="698" height="486" alt="Screenshot 2026-09-25 at 10 42 32 AM" src="https://github.com/user-attachments/assets/9f536420-0cd9-436e-812a-7b827d8bf248" />

Think about it, if there was no activation function then the entire neural network would be simplified into a series of linear functions and
we would never be able to fit any complex data.

That's all fine in well for a two input Neuron but imagine there were hundreds or even thousands of weights and inputs to a single Neuron. How
would we solve this? 

That's where metrices come in. If we place all the inputs into a matrix and all the weights into another. We can just multiply them together
and add them to a vector of biases. Then we take our results and pass them into our activation function.

<img width="864" height="238" alt="Screenshot 2026-09-25 at 11 09 01 AM" src="https://github.com/user-attachments/assets/170833e7-8cb4-4e04-a2ed-1046d01aacbe" />

# Implementation
Let's first create our own matrix library.

## Matrix Library
Using matrix library we can perform various operations on matrics of inputs and weights.

```rust
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
```

## Neural Network
Let's program our neural network. 

```rust
pub struct Network {
    layers:Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}
```

This is our network data strcuture. The layers are represented as a vector of the amount of neurons per layer. Weights, biases and inputs
are stored as a vector of type Matrix and finally we have got our activation function and learning rate.

Let's add a function to initialize our network.

```rust
use crate::activations::Activation;
use matrix::matrix::Matrix;

impl Network {
    pub fn new(layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate,
        }
    }
}
```

Now that we have implemented our neural network. Let's implement feed forward function. **Feed Forward** was a process of computing the output
or prediction from the input data by propagating it forward through the network layers.
