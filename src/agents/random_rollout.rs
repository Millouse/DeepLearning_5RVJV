use crate::traits::{Action, Agent, Env, Observation};
use rand::{RngExt, rng};

pub struct RandomRolloutAgent {
    n_simulations: usize,
}

impl RandomRolloutAgent {
    pub fn new(n_simulations: usize) -> Self {
        Self { n_simulations }
    }

    pub fn simulate(&self, env: &mut Box<dyn Env>) -> f32 {
        loop {
            if env.is_game_over() {
                break;
            }
            let mut rng = rand::rng();
            let legal_actions = env.legal_action();
            let index = rng.random_range(0..legal_actions.len());
            env.step(legal_actions[index]);
        }
        env.score(0) // 0 is player
    }
}

impl Agent for RandomRolloutAgent {
    fn select_action(
        &mut self,
        observation: &Observation,
        legal_actions: Vec<Action>,
        env: Option<&dyn Env>,
    ) -> Action {
        let mut mean_score: Vec<(f32, Action)> = Vec::with_capacity(legal_actions.len());
        let env = env.expect("RandomRollout without env is wrong.");
        for action in legal_actions {
            let mut total_score = 0.0;

            for i in 0..self.n_simulations {
                let mut clone_env: Box<dyn Env> = env.clone_env();
                let (_, _, done) = clone_env.step(action);
                if done {
                    total_score += clone_env.score(0);
                } else {
                    total_score += self.simulate(&mut clone_env);
                }
            }

            mean_score.push((total_score / self.n_simulations as f32, action));
        }
        mean_score.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        mean_score.last().unwrap().1
    }
}
