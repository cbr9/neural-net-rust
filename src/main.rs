use std::f64::consts::E;

fn main() {
    let neuron = Neuron {
        weights: [-6, 2],
        bias: 1,
    };
    println!("{}", neuron.feedforward([1, 3], ActivationFunction::Sigmoid));
    println!("{}", neuron.feedforward([1, 3], ActivationFunction::ReLU));
    println!("{}", neuron.feedforward([1, 3], ActivationFunction::HeavisideStep));
}

struct Neuron {
    weights: [i32; 2],
    bias: i32
}

fn dot_product(x: [i32; 2], y: [i32; 2]) -> i32 {
    let mut total: i32 = 0;
    for i in 0..2 {
        total += x[i] * y[i]
    }
    total
}

impl Neuron {
    fn feedforward(&self, inputs: [i32; 2], activation_function: ActivationFunction) -> f64 {
        let total= dot_product(self.weights, inputs) as f64;
       return activation_function.compute(total)
    }
}

enum ActivationFunction {
    Sigmoid,
    ReLU,
    HeavisideStep,
}

impl ActivationFunction {
    fn compute(&self, x: f64) -> f64 {
        return match self {
            self::ActivationFunction::Sigmoid => {
                1f64 / (1f64 + E.powf(-x))
            },
            self::ActivationFunction::ReLU => {
                if x <= 0f64 {
                    0f64
                } else {
                    x
                }
            },
            self::ActivationFunction::HeavisideStep => {
                if x < 0f64 {
                    0f64
                } else {
                    1f64
                }
            }
            _ => x
        }
    }
}