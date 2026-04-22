use burn::Tensor;
use burn::config::Config;
use burn::module::Module;
use burn::nn::{Linear, LinearConfig, Relu};
use burn::prelude::Backend;

#[derive(Module, Debug)]
pub struct MLP<B: Backend> {
    fc1: Linear<B>,
    fc2: Linear<B>,
    fc3: Linear<B>,
    activation: Relu,
}

#[derive(Config, Debug)]
pub struct MLPConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,
}

impl MLPConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> MLP<B> {
        MLP {
            fc1: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            fc2: LinearConfig::new(self.hidden_size, self.hidden_size).init(device),
            fc3: LinearConfig::new(self.hidden_size, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}

impl<B: Backend> MLP<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let input = self.fc1.forward(input);
        let input = self.activation.forward(input);
        let input = self.fc2.forward(input);
        let input = self.activation.forward(input);
        let input = self.fc3.forward(input);
        input
    }
}
