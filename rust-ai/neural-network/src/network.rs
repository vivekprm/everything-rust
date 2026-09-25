use crate::activations::Activation;
use matrix::matrix::Matrix;

#[derive(Builder)]
pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

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

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(
            self.layers[0] == inputs.data.len(),
            "Invalid Number of Inputs"
        );

        // holds the input for next layer of neural network
        let mut current = inputs;
        self.data = vec![current.clone()];

        // iterate over all layers and apply feed_forward algorithm
        for i in 0..self.layers.len() - 1 {
            current = self.weights[i]
                .dot_multiply(&current)
                .add(&self.biases[i])
                .map(self.activation.function);
            self.data.push(current.clone());
        }
        current
    }

    // inputs: represent the output of our feed_forward function.
    // targets: represents the expected output of our neural network
    pub fn back_propagate(&mut self, inputs: Matrix, targets: Matrix) {
        // calculate cost, how off our input was from the expected output
        let mut errors = targets.subtract(&inputs);

        // Whole point of neural network learning is to minimize the cost function to as little as
        // possible
        // Next search which part of our Network was most responsible for producing the incorrect
        // output
        let mut gradients = inputs.clone().map(self.activation.derivative);

        // Iterate layer by layer multiplying our activation function by it's derivative to access
        // the weight and biases
        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients
                .elementwise_multiply(&errors)
                .map(|x: &f64| x * 0.5);
            self.weights[i] =
                self.weights[i].add(&gradients.dot_multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            // Then we will multiply them by our error to update our neurons proportionately.
            // The higher the gradient the more responsible that particular neuron is to producing
            // the output. So the more wrong a neuron is the more it will get updated.
            errors = self.weights[i].transpose().dot_multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    // The main loop of our program.
    // We are simply running feed_forward and back_propagate hundreds or thousands of times until
    // our network is finally tuned and ready to be used.
    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                println!("Epochs {} of {}", i, epochs);
            }
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(Matrix::from(inputs[j].clone()));
                self.back_propagate(outputs, Matrix::from(targets[j].clone()));
            }
        }
    }
}
