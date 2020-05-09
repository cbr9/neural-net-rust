use std::f64::consts::E;
fn main() {
    let neuron = Neuron {
        weights: vec![3, 2],
        bias: 1
    };
    println!("{}", neuron.feedforward(vec![1, 3], ActivationFunction::Sigmoid));
    println!("{}", neuron.feedforward(vec![1, 3], ActivationFunction::ReLU));
    println!("{}", neuron.feedforward(vec![1, 3], ActivationFunction::HeavisideStep));
}

fn dot_product(x: &[i32], y: &[i32]) -> i32 {
    let mut total = 0;
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
    weights: Vec<i32>,
    bias: i32
}

impl Neuron {
    fn feedforward(&self, inputs: Vec<i32>, activation_function: ActivationFunction) -> f32 {
        let total= dot_product(self.weights.as_slice(), inputs.as_slice()) as f32;
       return activation_function.compute(total)
    }
}

enum ActivationFunction {
    Sigmoid,
    ReLU,
    HeavisideStep,
}

impl ActivationFunction {
    fn compute(&self, x: f32) -> f32 {
        return match self {
            self::ActivationFunction::Sigmoid => {
                1f32 / (1f32 + E.powf(-x as f64) as f32)  // 1 / (1 + e^-x)
            },
            self::ActivationFunction::ReLU => {
                if x <= 0f32 {
                    0f32
                } else {
                    x
                }
            },
            self::ActivationFunction::HeavisideStep => {
                if x < 0f32 {
                    0f32
                } else {
                    1f32
                }
            }
        }
    }
}