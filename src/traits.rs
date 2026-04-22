use std::ptr::null;

/// Flat float vector for neural network
pub type Observation = Vec<f32>;

/// Action index
pub type Action = usize;

/// Reward
pub type Reward = f32;

pub trait Env {
    /// Reset environment
    fn reset(&mut self) -> Observation;

    /// Apply an action to an env, return (next_observation, reward, is_done)
    fn step(&mut self, action: Action) -> (Observation, Reward, bool);

    /// List of all legal actions
    fn legal_action(&self) -> Vec<Action>;

    /// Return true if game is over, else false
    fn is_game_over(&self) -> bool;

    /// Return 0 or 1 for player 1 and 2
    fn current_player(&self) -> usize;

    /// Size of possible actions
    fn num_actions(&self) -> usize;

    /// Size of observation vector
    fn observation_size(&self) -> usize;

    /// Score of a player at the end of game
    fn score(&self, player: usize) -> f32;

    /// Clone env
    fn clone_env(&self) -> Box<dyn Env>;
}

pub trait Agent {
    /// Give current obs and legal actions, return the chosen action
    fn select_action(
        &mut self,
        observation: &Observation,
        legal_actions: Vec<Action>,
        env: Option<&dyn Env>,
    ) -> Action;
}

pub trait TrainableAgent: Agent {
    /// Store the reward
    fn store_reward(&mut self, reward: f32);

    /// Train step
    fn train_step(&mut self);
}

#[derive(Clone)]
pub struct Transition {
    pub observation: Observation,
    pub action: Action,
    pub reward: Reward,
    pub next_observation: Observation,
    pub done: bool,
    pub legal_actions: Vec<Action>,
    pub next_legal_actions: Vec<Action>,
}

pub struct EpisodeResult {
    pub score: f32,
    pub score_player_2: f32,
    pub num_steps: usize,
    pub move_time_ms: f64, // Average time for a step
}

pub struct RunStats {
    pub mean_score: f32,
    pub mean_score_player_2: f32,
    pub mean_steps: f32,
    pub mean_move_time_ms: f64,
    pub n_episodes: usize,
}
