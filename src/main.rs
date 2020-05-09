use std::f64::consts::E;
use std::borrow::Borrow;

fn main() {
    let weights = vec![0f32, 1f32];
    let bias = 0f32;
    let nn = NeuralNetwork::new(weights, bias, 2, ActivationFn::Sigmoid);
    let inputs = vec![2f32, 3f32];
    println!("{}", nn.feedforward(inputs))
}

fn dot_product(x: &[f32], y: &[f32]) -> f32 {
    let mut total = 0f32;
    if x.len() == y.len() {
        for i in 0..x.len() {
            total += x[i] * y[i];
        }
        return total;
    }
    panic!("vectors don't have the same dimensions")
}

#[allow(dead_code)]
struct Neuron {
    weights: Vec<f32>,
    bias: f32
}

impl Neuron {
    fn new(weights: Vec<f32>, bias: f32) -> Neuron {
        return Neuron {
            weights, bias
        }
    }
    fn feedforward(&self, inputs: &Vec<f32>, activation_function: &ActivationFn) -> f32 {
        let total= dot_product(self.weights.as_slice(), inputs.as_slice()) as f32;
       return activation_function.compute(total)
    }
}

enum LossFn {
    MSE
}

impl LossFn {
    fn compute(&self) {
        return match self {
            self::LossFn::MSE => 
        }
    }
}

enum ActivationFn {
    Sigmoid,
    ReLU,
    HeavisideStep,
}

impl ActivationFn {
    fn compute(&self, x: f32) -> f32 {
        return match self {
            self::ActivationFn::Sigmoid => {
                1f32 / (1f32 + E.powf(-x as f64) as f32)  // 1 / (1 + e^-x)
            },
            self::ActivationFn::ReLU => {
                if x <= 0f32 {
                    0f32
                } else {
                    x
                }
            },
            self::ActivationFn::HeavisideStep => {
                if x < 0f32 {
                    0f32
                } else {
                    1f32
                }
            }
        }
    }
}

struct NeuralNetwork {
    weights: Vec<f32>,
    bias: f32,
    hidden_layer: Vec<Neuron>,
    output_neuron: Neuron,
    activation_function: ActivationFn,
}

impl NeuralNetwork {
    fn new(weights: Vec<f32>, bias: f32, number_hidden_neurons: i32, activation_function: ActivationFn) -> NeuralNetwork {
        let mut hidden_neurons: Vec<Neuron> = Vec::new();
        for _ in 0..number_hidden_neurons {
            hidden_neurons.push(Neuron::new(weights.clone(), bias))
        }

        return NeuralNetwork {
            weights: weights.clone(),
            bias,
            hidden_layer: hidden_neurons,
            output_neuron: Neuron::new(weights, bias),
            activation_function,
        }
    }

    fn mse(y_true: Vec<i32>, y_pred: Vec<i32>) -> f32 {

    }
    
    fn feedforward(&self, inputs: Vec<f32>) -> f32 {
        let mut hidden_outputs : Vec<f32> = Vec::new();
        for neuron in self.hidden_layer.iter() {
            hidden_outputs.push(neuron.feedforward(&inputs, self.activation_function.borrow()))
        }
        return self.output_neuron.feedforward(&hidden_outputs, self.activation_function.borrow())
    }
}

