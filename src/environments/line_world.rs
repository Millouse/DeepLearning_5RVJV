use crate::traits::{Action, Env, Observation, Reward};

const ACTIONS: &[Action] = &[0, 1];

pub struct LineWorld {
    /// Number of cells
    size: usize,

    /// Current agent pos
    agent_pos: usize,

    /// Starting pos
    start_pos: usize,

    /// Max step for an episode
    max_steps: usize,

    /// Current step
    current_step: usize,

    /// Is the game finished
    game_over: bool,

    /// Score
    final_score: f32,
    
    /// Is going out of the board is killing the player
    out_of_bounds_is_killing: bool,
}

impl LineWorld {
    pub fn new(size: usize, start_pos: usize, max_steps: usize, out_of_bounds_is_killing: bool) -> Self {
        Self {
            size,
            agent_pos: start_pos,
            start_pos,
            max_steps,
            current_step: 0,
            game_over: false,
            final_score: 0.0,
            out_of_bounds_is_killing,
        }
    }

    pub fn pos_to_observation(&self) -> Observation {
        let mut obs = vec![0.0f32; self.size];
        obs[self.agent_pos] = 1.0;
        obs
    }
}

impl Env for LineWorld {
    fn reset(&mut self) -> Observation {
        self.agent_pos = self.start_pos;
        self.current_step = 0;
        self.game_over = false;
        self.final_score = 0.0;
        self.pos_to_observation()
    }

    fn step(&mut self, action: Action) -> (Observation, Reward, bool) {
        if self.game_over {
            panic!("Game over")
        }

        self.current_step += 1;

        match action {
            // Goes to left
            0 => {
                if self.out_of_bounds_is_killing {
                    if self.agent_pos == 0 {
                        self.game_over = true;
                        self.final_score = 0.0;
                        return (self.pos_to_observation(), 0.0, true);
                    }
                } else {
                    if self.agent_pos != 0 {
                        self.agent_pos -= 1;
                    }
                }
            }
            1 => {
                self.agent_pos += 1;
            }
            _ => panic!("Invalid action : {}", action),
        }

        if self.agent_pos == self.size - 1 {
            self.game_over = true;
            self.final_score = 1.0;
            return (self.pos_to_observation(), 1.0, true);
        }

        if self.current_step == self.max_steps {
            self.game_over = true;
            self.final_score = 0.0;
            return (self.pos_to_observation(), 0.0, true);
        }

        // -0.01 to penalize
        (self.pos_to_observation(), -0.01, false)
    }

    fn legal_action(&self) -> &[Action] {
        ACTIONS
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn current_player(&self) -> usize {
        0
    }

    fn num_actions(&self) -> usize {
        2
    }

    fn observation_size(&self) -> usize {
        self.size
    }

    fn score(&self, player: usize) -> f32 {
        self.final_score
    }
}
