use crate::models::mlp::{MLP, MLPConfig};
use crate::traits::{Action, Agent, Env, Observation, TrainableAgent};
use burn::Tensor;
use burn::optim::{Adam, AdamConfig, AdamState, GradientsParams, Optimizer};
use burn::tensor::activation::softmax;
use burn::tensor::backend::AutodiffBackend;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::rng;
use rand::rngs::ThreadRng;

type MyOptimizer<B> = burn::optim::adaptor::OptimizerAdaptor<burn::optim::Adam, MLP<B>, B>;

pub struct ReinforceAgent<B: AutodiffBackend> {
    /// Network
    network: MLP<B>,

    /// Optimizer
    optimizer: MyOptimizer<B>,

    /// Log probabilities of the episode
    log_probs: Vec<Tensor<B, 1>>,

    /// Reward of the episode
    rewards: Vec<f32>,

    /// hyperparameter
    gamma: f32,

    /// Device
    device: B::Device,

    /// Evaluation mode
    pub(crate) eval_mode: bool,

    /// Number of possible actions
    num_actions: usize,

    /// Learning rate
    learning_rate: f64,
}

impl<B: AutodiffBackend> ReinforceAgent<B> {
    pub fn new(
        observation_size: usize,
        num_actions: usize,
        hidden_size: usize,
        gamma: f32,
        device: B::Device,
        learning_rate: f64,
    ) -> Self {
        let network = MLPConfig::new(observation_size, hidden_size, num_actions).init(&device);
        let optimizer = AdamConfig::new().init();

        Self {
            network,
            optimizer,
            log_probs: vec![],
            rewards: vec![],
            gamma,
            device,
            eval_mode: false,
            num_actions,
            learning_rate,
        }
    }

    pub fn select_action(
        &mut self,
        observation: &Observation,
        legal_actions: Vec<Action>,
        env: Option<&dyn Env>,
    ) -> Action {
        let tensor =
            Tensor::<B, 1>::from_floats(observation.as_slice(), &self.device).unsqueeze::<2>();

        let logits = self.network.forward(tensor);

        let mut mask = vec![f32::NEG_INFINITY; self.num_actions];
        for &a in &legal_actions {
            mask[a] = 0.0;
        }
        let mask_tensor =
            Tensor::<B, 1>::from_floats(mask.as_slice(), &self.device).unsqueeze::<2>();
        let logits = logits + mask_tensor;

        let probs_vec = softmax(logits.clone(), 1)
            .detach()
            .reshape([self.num_actions])
            .into_data()
            .to_vec::<f32>()
            .unwrap();

        let action = if self.eval_mode {
            *legal_actions
                .iter()
                .max_by(|&&a, &&b| probs_vec[a].partial_cmp(&probs_vec[b]).unwrap())
                .unwrap()
        } else {
            let weights: Vec<f32> = legal_actions.iter().map(|&a| probs_vec[a]).collect();
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = rand::rng();
            legal_actions[dist.sample(&mut rng)]
        };

        if !self.eval_mode {
            let log_prob = softmax(logits, 1)
                .reshape([self.num_actions])
                .select(0, Tensor::from_ints([action as i32], &self.device))
                .log();
            self.log_probs.push(log_prob);
        }
        action
    }

    pub fn store_reward(&mut self, reward: f32) {
        self.rewards.push(reward);
    }

    pub fn train_step(&mut self) {
        let mut returns = vec![0.0f32; self.rewards.len()];
        let mut running = 0.0f32;

        for i in (0..self.rewards.len()).rev() {
            running = self.rewards[i] + self.gamma * running;
            returns[i] = running;
        }
        let mean = returns.iter().sum::<f32>() / returns.len() as f32;
        let std =
            (returns.iter().map(|r| (r - mean).powi(2)).sum::<f32>() / returns.len() as f32).sqrt();
        let returns: Vec<f32> = returns.iter().map(|r| (r - mean) / (std + 1e-8)).collect();
        let log_probs_tensor = Tensor::cat(self.log_probs.clone(), 0);
        let returns_tensor = Tensor::<B, 1>::from_floats(returns.as_slice(), &self.device);
        let loss = (log_probs_tensor * returns_tensor).neg().mean();
        let gradients = loss.backward();
        let gradient_params = GradientsParams::from_grads(gradients, &self.network);
        self.network =
            self.optimizer
                .step(self.learning_rate, self.network.clone(), gradient_params);
        self.log_probs.clear();
        self.rewards.clear();
    }
}

impl<B: AutodiffBackend> Agent for ReinforceAgent<B> {
    fn select_action(
        &mut self,
        obs: &Observation,
        legal_actions: Vec<Action>,
        env: Option<&dyn Env>,
    ) -> Action {
        self.select_action(obs, legal_actions, env)
    }
}

impl<B: AutodiffBackend> TrainableAgent for ReinforceAgent<B> {
    fn store_reward(&mut self, reward: f32) {
        self.rewards.push(reward);
    }

    fn train_step(&mut self) {
        self.train_step();
    }
}
