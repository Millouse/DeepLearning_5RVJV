use crate::traits::{Action, Env, Observation, Reward};

const WINNING_COMBINAISONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Clone)]
pub struct TicTacToe {
    /// Board, 9 cells, 0 for empty, 1 for player, 2 for ai
    board: Vec<u8>,

    /// Current player turn, 0 for player, 1 for ia
    current_player_turn: usize,

    /// Is the game finished
    game_over: bool,

    /// Who won, 0 for draw, 1 for player, 2 for ai
    winner: usize,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: vec![0u8; 9],
            current_player_turn: 0,
            game_over: false,
            winner: 0,
        }
    }

    pub fn to_observation(&self) -> Observation {
        // 27 because 3x 9 pos
        let mut obs = vec![0.0f32; 27];

        for i in 0..9 {
            match self.board[i] {
                0 => obs[i] = 1.0,      // plan 0 : empty
                1 => obs[9 + i] = 1.0,  // plan 1 : player
                2 => obs[18 + i] = 1.0, // plan 2 : ai
                _ => {}
            }
        }
        obs
    }

    fn check_winner(&mut self) -> u8 {
        for combi in WINNING_COMBINAISONS.iter() {
            if self.board[combi[0]] != 0 {
                if (self.board[combi[0]] == self.board[combi[1]])
                    && (self.board[combi[1]] == self.board[combi[2]])
                {
                    return self.board[combi[0]];
                }
            }
        }
        0
    }
}

impl Env for TicTacToe {
    fn reset(&mut self) -> Observation {
        self.board = vec![0u8; 9];
        self.current_player_turn = 0;
        self.game_over = false;
        self.winner = 0;
        self.to_observation()
    }

    fn step(&mut self, action: Action) -> (Observation, Reward, bool) {
        if self.game_over {
            panic!("TicTacToe game over");
        }

        self.board[action] = (self.current_player_turn + 1) as u8;

        let winner = self.check_winner();
        if winner != 0 {
            self.game_over = true;
            self.winner = winner as usize;
        } else if self.legal_action().is_empty() {
            self.game_over = true;
            self.winner = 0;
        }

        let reward: f32 = if self.game_over {
            if self.winner == 0 {
                0.0
            } else if self.winner == self.current_player_turn + 1 {
                1.0
            } else {
                -1.0
            }
        } else {
            0.0
        };

        self.current_player_turn = 1 - self.current_player_turn;

        (self.to_observation(), reward, self.game_over)
    }

    fn legal_action(&self) -> Vec<Action> {
        let mut legal_actions = vec![];
        for index in 0..9 {
            if self.board[index] == 0 {
                legal_actions.push(index);
            }
        }
        legal_actions
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn current_player(&self) -> usize {
        self.current_player_turn
    }

    fn num_actions(&self) -> usize {
        9
    }

    fn observation_size(&self) -> usize {
        27
    }

    fn score(&self, player: usize) -> f32 {
        match self.winner {
            0 => 0.5,
            1 => {
                if player == 0 {
                    1.0
                } else {
                    -1.0
                }
            }
            2 => {
                if player == 0 {
                    -1.0
                } else {
                    1.0
                }
            }
            _ => panic!("TicTacToe impossible score"),
        }
    }

    fn clone_env(&self) -> Box<dyn Env> {
        Box::new(self.clone())
    }
}
