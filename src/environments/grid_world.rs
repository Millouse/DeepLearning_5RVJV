use crate::traits::{Action, Env, Observation, Reward};
use burn::tensor::Slice;

#[derive(Clone)]
pub struct GridWorld {
    /// Width of the grid
    width: usize,

    /// Height of the grid
    height: usize,

    /// Position of the agent
    agent_pos: (usize, usize),

    /// Starting pos for reset
    starting_pos: (usize, usize),

    /// Current step
    current_step: usize,

    /// Max step for an episode
    max_steps: usize,

    /// Is the game finished
    game_over: bool,

    /// Final score
    final_score: f32,

    /// Is going out of the board is killing the player
    out_of_bounds_is_killing: bool,
}

impl GridWorld {
    pub fn new(
        width: usize,
        height: usize,
        max_steps: usize,
        out_of_bounds_is_killing: bool,
    ) -> Self {
        Self {
            width,
            height,
            agent_pos: (0, 0),
            starting_pos: (0, 0),
            current_step: 0,
            game_over: false,
            final_score: 0.0,
            max_steps,
            out_of_bounds_is_killing,
        }
    }

    pub fn pos_to_observation(&self) -> Observation {
        let mut obs = vec![0.0f32; self.width * self.height];
        obs[(self.agent_pos.1 * self.width) + self.agent_pos.0] = 1.0;
        obs
    }
}

impl Env for GridWorld {
    fn reset(&mut self) -> Observation {
        self.current_step = 0;
        self.agent_pos = self.starting_pos;
        self.game_over = false;
        self.final_score = 0.0;
        self.pos_to_observation()
    }

    fn step(&mut self, action: Action) -> (Observation, Reward, bool) {
        if self.game_over {
            panic!("Game over!");
        }
        self.current_step += 1;

        match action {
            // Up
            0 => {
                if self.out_of_bounds_is_killing {
                    if self.agent_pos.1 == 0 {
                        self.game_over = true;
                        self.final_score = 0.0;
                        return (self.pos_to_observation(), 0.0, true);
                    } else {
                        self.agent_pos.1 -= 1
                    }
                } else {
                    if self.agent_pos.1 != 0 {
                        self.agent_pos.1 -= 1;
                    }
                }
            }

            // Bas
            1 => {
                if self.out_of_bounds_is_killing {
                    if self.agent_pos.1 == self.height - 1 {
                        self.game_over = true;
                        self.final_score = 0.0;
                        return (self.pos_to_observation(), 0.0, true);
                    } else {
                        self.agent_pos.1 += 1;
                    }
                } else {
                    if self.agent_pos.1 != self.height - 1 {
                        self.agent_pos.1 += 1;
                    }
                }
            }

            // Left
            2 => {
                if self.out_of_bounds_is_killing {
                    if self.agent_pos.0 == 0 {
                        self.game_over = true;
                        self.final_score = 0.0;
                        return (self.pos_to_observation(), 0.0, true);
                    } else {
                        self.agent_pos.0 -= 1;
                    }
                } else {
                    if self.agent_pos.0 != 0 {
                        self.agent_pos.0 -= 1;
                    }
                }
            }

            // Right
            3 => {
                if self.out_of_bounds_is_killing {
                    if self.agent_pos.0 == self.width - 1 {
                        self.game_over = true;
                        self.final_score = 0.0;
                        return (self.pos_to_observation(), 0.0, true);
                    } else {
                        self.agent_pos.0 += 1;
                    }
                } else {
                    if self.agent_pos.0 != self.width - 1 {
                        self.agent_pos.0 += 1;
                    }
                }
            }
            _ => panic!("Unhandled action {}", action),
        }

        if self.agent_pos == (self.width - 1, self.height - 1) {
            self.game_over = true;
            self.final_score = 1.0;
            return (self.pos_to_observation(), 1.0, true);
        }

        if self.current_step >= self.max_steps {
            self.game_over = true;
            self.final_score = 0.0;
            return (self.pos_to_observation(), 0.0, true);
        }

        (self.pos_to_observation(), -0.01, false)
    }

    fn legal_action(&self) -> Vec<Action> {
        vec![0, 1, 2, 3]
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn current_player(&self) -> usize {
        0
    }

    fn num_actions(&self) -> usize {
        4
    }

    fn observation_size(&self) -> usize {
        self.width * self.height
    }

    fn score(&self, player: usize) -> f32 {
        self.final_score
    }

    fn clone_env(&self) -> Box<dyn Env> {
        Box::new(self.clone())
    }
}
