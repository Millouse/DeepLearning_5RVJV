use crate::traits::{Action, Env, Observation, Reward};

#[derive(Clone)]
pub struct TicTacToe {
    /// Width of the grid
    width: usize,

    /// Height of the grid
    height: usize,

    /// Actual step of the episode
    current_step: usize,

    /// Current player turn, 0 for player, 1 for ia
    current_player_turn: usize,

    /// Max step for an episode
    max_steps : usize,

    /// Is the game finished
    game_over : bool,

    /// Final score
    final_score : f32,
}

impl TicTacToe {
    pub fn new(width: usize, height: usize, max_steps: usize) -> Self {
        Self {
            width,
            height,
            current_step: 0,
            current_player_turn: 0,
            max_steps,
            game_over: false,
            final_score: 0.0,
        }
    }
}

impl Env for TicTacToe {
    fn reset(&mut self) -> Observation {
        todo!()
    }

    fn step(&mut self, action: Action) -> (Observation, Reward, bool) {
        todo!()
    }

    fn legal_action(&self) -> Vec<Action> {
        todo!()
    }

    fn is_game_over(&self) -> bool {
        todo!()
    }

    fn current_player(&self) -> usize {
        todo!()
    }

    fn num_actions(&self) -> usize {
        todo!()
    }

    fn observation_size(&self) -> usize {
        todo!()
    }

    fn score(&self, player: usize) -> f32 {
        todo!()
    }

    fn clone_env(&self) -> Box<dyn Env> {
        Box::new(self.clone())
    }
}