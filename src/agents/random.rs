use crate::traits::{Action, Agent, Env, Observation, TrainableAgent};
use rand::{RngExt, rng};

pub struct RandomAgent;

impl TrainableAgent for RandomAgent {
    fn store_reward(&mut self, _reward: f32) {}
    fn train_step(&mut self) {}
}

impl Agent for RandomAgent {
    fn select_action(
        &mut self,
        observation: &Observation,
        legal_actions: Vec<Action>,
        _env: Option<&dyn Env>,
    ) -> Action {
        let mut rng = rand::rng();
        let index = rng.random_range(0..legal_actions.len());
        legal_actions[index]
    }
}
