use crate::traits::{Action, Env, Observation, Reward};

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Egg(usize),
    Tadpole(usize),
    Frog(usize),
}

impl Cell {
    pub fn owner(&self) -> Option<usize> {
        match self {
            Cell::Egg(o) | Cell::Tadpole(o) | Cell::Frog(o) => Some(*o),
            Cell::Empty => None,
        }
    }

    pub fn same_type(&self, other: &Cell) -> bool {
        matches!(
            (self, other),
            (Cell::Egg(_), Cell::Egg(_))
                | (Cell::Tadpole(_), Cell::Tadpole(_))
                | (Cell::Frog(_), Cell::Frog(_))
        )
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn evolve(&mut self) -> () {
        match self {
            Cell::Empty => {}
            Cell::Egg(owner) => *self = Cell::Tadpole(*owner),
            Cell::Tadpole(owner) => *self = Cell::Frog(*owner),
            Cell::Frog(owner) => *self = Cell::Egg(*owner),
        }
    }
}

#[derive(Clone)]
pub struct Pond {
    /// Board of the game
    board: Vec<Cell>,

    /// Current player
    current_player: usize,

    /// Winner, 0 is draw, 1 is player, 2 is enemy
    winner: usize,

    /// Egg left to play
    egg_in_spawn: [usize; 2],

    /// Score of players
    score: [usize; 2],

    /// Is the game finished
    game_over: bool,
}

impl Pond {
    pub fn new() -> Self {
        Self {
            board: vec![Cell::Empty; 16],
            current_player: 0,
            winner: 0,
            egg_in_spawn: [13, 13],
            score: [0, 0],
            game_over: false,
        }
    }

    pub fn to_observation(&self) -> Observation {
        // 16 cases × 7 + 2 egg_in_spawn + 2 scores = 116
        let mut obs = vec![0.0f32; 16 * 7 + 2 + 2];

        for i in 0..16 {
            let offset = match self.board[i] {
                Cell::Empty => 0,
                Cell::Egg(0) => 1,
                Cell::Egg(_) => 2,
                Cell::Tadpole(0) => 3,
                Cell::Tadpole(_) => 4,
                Cell::Frog(0) => 5,
                Cell::Frog(_) => 6,
            };
            obs[i * 7 + offset] = 1.0;
        }

        obs[16 * 7] = self.egg_in_spawn[0] as f32 / 13.0;
        obs[16 * 7 + 1] = self.egg_in_spawn[1] as f32 / 13.0;

        obs[16 * 7 + 2] = self.score[0] as f32 / 10.0;
        obs[16 * 7 + 3] = self.score[1] as f32 / 10.0;

        obs
    }

    pub fn orthogonal_neighbors(&self, src: usize) -> Vec<usize> {
        let mut neighbors = vec![];
        let x = src % 4;
        let y = src / 4;

        if y > 0 {
            neighbors.push(src - 4);
        } // up
        if y < 3 {
            neighbors.push(src + 4);
        } // down
        if x > 0 {
            neighbors.push(src - 1);
        } // left
        if x < 3 {
            neighbors.push(src + 1);
        } // right

        neighbors
    }

    pub fn apply_develop(&mut self, src: usize) {
        let neighbors = self.orthogonal_neighbors(src);
        for neighbor in neighbors {
            self.board[neighbor].evolve();
        }
    }

    pub fn apply_score(&mut self) {
        use std::collections::HashSet;
        let mut to_score: HashSet<usize> = HashSet::new();

        let lines: Vec<Vec<usize>> = vec![
            // horizontal lines
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
            // vertical lines
            vec![0, 4, 8, 12],
            vec![1, 5, 9, 13],
            vec![2, 6, 10, 14],
            vec![3, 7, 11, 15],
        ];

        for line in &lines {
            for window in line.windows(3) {
                let a = self.board[window[0]];
                let b = self.board[window[1]];
                let c = self.board[window[2]];

                if !a.is_empty() && a.same_type(&b) && b.same_type(&c) {
                    to_score.insert(window[0]);
                    to_score.insert(window[1]);
                    to_score.insert(window[2]);
                }
            }
        }
        for idx in to_score {
            if let Some(owner) = self.board[idx].owner() {
                self.score[owner] += 1;
            }
            self.board[idx] = Cell::Empty;
        }
    }

    pub fn get_cell_index(&self, index: usize) -> Cell{
        self.board[index]
    }
}

impl Env for Pond {
    fn reset(&mut self) -> Observation {
        self.current_player = 0;
        self.winner = 0;
        self.egg_in_spawn = [13, 13];
        self.score = [0, 0];
        self.game_over = false;
        self.board = vec![Cell::Empty; 16];
        self.to_observation()
    }

    fn step(&mut self, action: Action) -> (Observation, Reward, bool) {
        if self.game_over {
            panic!("Pond game over!");
        }

        let dst = if action < 16 {
            self.board[action] = Cell::Egg(self.current_player);
            self.egg_in_spawn[self.current_player] -= 1;
            action
        } else if action < 16 + 256 {
            let tmp = action - 16;
            let src = tmp / 16;
            let dst = tmp % 16;
            self.board[dst] = self.board[src];
            self.board[src] = Cell::Empty;
            dst
        } else {
            let tmp = action - (16 + 256);
            let src = tmp / 16;
            let dst = tmp % 16;
            self.board[dst] = self.board[src];
            self.board[src] = Cell::Empty;
            dst
        };

        self.apply_develop(dst);
        self.apply_score();

        if self.score[0] >= 10 || self.score[1] >= 10 {
            self.game_over = true;
            self.winner = if self.score[0] > self.score[1] { 1 } else { 2 };
            let reward = if self.winner == self.current_player + 1 {
                1.0
            } else {
                -1.0
            };
            return (self.to_observation(), reward, true);
        }

        self.current_player = 1 - self.current_player;

        if self.legal_action().is_empty() {
            self.game_over = true;
            self.winner = 1 - self.current_player + 1;
            return (self.to_observation(), 1.0, true);
        }
        (self.to_observation(), 0.0, false)
    }

    fn legal_action(&self) -> Vec<Action> {
        let mut legal_action: Vec<Action> = Vec::new();
        if self.egg_in_spawn[self.current_player] > 0 {
            for dest in 0..16 {
                if self.board[dest] == Cell::Empty {
                    legal_action.push(dest);
                }
            }
        }

        for src in 0..16 {
            if let Cell::Tadpole(owner) = self.board[src] {
                if owner == self.current_player {
                    for dst in self.orthogonal_neighbors(src) {
                        if self.board[dst] == Cell::Empty {
                            legal_action.push(16 + src * 16 + dst);
                        }
                    }
                }
            }
        }

        for src in 0..16 {
            if let Cell::Frog(owner) = self.board[src] {
                if owner == self.current_player {
                    let x = src % 4;
                    let y = src / 4;

                    for dst in self.orthogonal_neighbors(src) {
                        if self.board[dst] == Cell::Empty {
                            legal_action.push(16 + 256 + src * 16 + dst);
                        }
                    }

                    if y >= 2 && self.board[src - 8].is_empty() {
                        legal_action.push(16 + 256 + src * 16 + (src - 8));
                    }
                    if y <= 1 && self.board[src + 8].is_empty() {
                        legal_action.push(16 + 256 + src * 16 + (src + 8));
                    }
                    if x >= 2 && self.board[src - 2].is_empty() {
                        legal_action.push(16 + 256 + src * 16 + (src - 2));
                    }
                    if x <= 1 && self.board[src + 2].is_empty() {
                        legal_action.push(16 + 256 + src * 16 + (src + 2));
                    }
                }
            }
        }
        legal_action
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn current_player(&self) -> usize {
        self.current_player
    }

    fn num_actions(&self) -> usize {
        16 + 16 * 16 + 16 * 16 // 16 for place egg, 16 x 16 for move tadpole, 16 x 16 for move frog
    }

    fn observation_size(&self) -> usize {
        16 * 7 + 2 + 2 // Empty, Egg(0), Egg(1), Tadpole(0), Tadpole(1), Frog(0), Frog(1) + egg_in_spawn + score
    }

    fn score(&self, player: usize) -> f32 {
        match self.winner {
            0 => 0.0,
            w if w == player + 1 => 1.0,
            _ => -1.0,
        }
    }

    fn clone_env(&self) -> Box<dyn Env> {
        Box::new(self.clone())
    }
}
