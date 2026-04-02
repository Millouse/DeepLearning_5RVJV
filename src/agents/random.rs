use crate::traits::{Action, Agent, Observation};
use rand::{RngExt, rng};

pub struct RandomAgent;

impl Agent for RandomAgent {
    fn select_action(&mut self, observation: &Observation, legal_actions: &[Action]) -> Action {
        let mut rng = rand::rng();
        let index = rng.random_range(0..legal_actions.len());
        legal_actions[index]
    }
}
