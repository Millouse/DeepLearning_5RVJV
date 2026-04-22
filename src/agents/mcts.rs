use crate::traits::{Action, Agent, Env, Observation};
use rand::RngExt;

struct MCTSNode {
    /// Number of time the node got visited
    visits: u32,

    /// Sums of scores from this node
    total_value: f32,

    /// Action who lead to this node from parent
    action: Option<Action>,

    /// Vec of all children of this node
    children: Vec<MCTSNode>,

    /// Vec of non-tried action
    untried_actions: Vec<Action>,

    /// Current player
    current_player: usize,
}

impl MCTSNode {
    fn new(action: Option<Action>, player: usize, untried_actions: Vec<Action>) -> Self {
        Self {
            visits: 0,
            total_value: 0.0,
            action,
            children: vec![],
            untried_actions,
            current_player: player,
        }
    }

    fn ucb1(&self, parent_visits: u32, c: f32) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }
        (self.total_value / self.visits as f32)
            + c * ((parent_visits as f32).ln() / self.visits as f32).sqrt()
    }

    fn is_fully_expanded(&self) -> bool {
        self.untried_actions.is_empty()
    }
}

pub struct MCTS {
    /// Number of simulations
    n_simulations: usize,

    /// Constant of exploration UCB1 : mean + c * (sqrt(len(total_value) / visits))
    c: f32,
}

impl MCTS {
    pub fn new(n_simulations: usize, c: f32) -> Self {
        MCTS { n_simulations, c }
    }

    fn select<'a>(&self, node: &'a mut MCTSNode, env: &mut Box<dyn Env>) -> &'a mut MCTSNode {
        if !node.is_fully_expanded() || env.is_game_over() {
            return node;
        }

        let best_idx = node
            .children
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.ucb1(node.visits, self.c)
                    .partial_cmp(&b.ucb1(node.visits, self.c))
                    .unwrap()
            })
            .map(|(i, _)| i)
            .unwrap();

        let (_, _, done) = env.step(node.children[best_idx].action.unwrap());
        self.select(&mut node.children[best_idx], env)
    }

    fn expand(node: &mut MCTSNode, env: &mut Box<dyn Env>) -> () {
        let player = env.current_player();

        let action = node.untried_actions.pop().unwrap();

        let (_, _, done) = env.step(action);

        let child_untried_actions = if done { vec![] } else { env.legal_action() };

        let new_node = MCTSNode::new(Some(action), player, child_untried_actions);

        node.children.push(new_node);
    }
    pub fn simulate(env: &mut Box<dyn Env>, player: usize) -> f32 {
        loop {
            if env.is_game_over() {
                break;
            }
            let mut rng = rand::rng();
            let legal_actions = env.legal_action();
            let index = rng.random_range(0..legal_actions.len());
            env.step(legal_actions[index]);
        }
        env.score(player)
    }

    fn backpropagate(node: &mut MCTSNode, value: f32, root_player: usize) {
        node.visits += 1;

        if node.current_player == root_player {
            node.total_value += value;
        } else {
            node.total_value -= value;
        }

        for child in node.children.iter_mut() {
            if child.visits > 0 {
                Self::backpropagate(child, value, root_player);
            }
        }
    }

    fn run_simulation(&self, root: &mut MCTSNode, env: &dyn Env, root_player: usize) {
        let mut cloned_env = env.clone_env();
        let mut path: Vec<usize> = vec![]; // indices des enfants choisis

        // SELECT — descendre avec indices
        let mut node_ref = &mut *root;
        loop {
            if cloned_env.is_game_over() || !node_ref.is_fully_expanded() {
                break;
            }
            let best_idx = node_ref
                .children
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    a.ucb1(node_ref.visits, self.c)
                        .partial_cmp(&b.ucb1(node_ref.visits, self.c))
                        .unwrap()
                })
                .map(|(i, _)| i)
                .unwrap();
            cloned_env.step(node_ref.children[best_idx].action.unwrap());
            path.push(best_idx);
            node_ref = &mut node_ref.children[best_idx];
        }

        // EXPAND
        if !cloned_env.is_game_over() {
            Self::expand(node_ref, &mut cloned_env);
            let new_idx = node_ref.children.len() - 1;
            path.push(new_idx);
        }

        // SIMULATE
        let value = Self::simulate(&mut cloned_env, root_player);

        // BACKPROPAGATE — remonter le chemin
        let mut node = &mut *root;
        node.visits += 1;
        node.total_value += if node.current_player == root_player {
            value
        } else {
            -value
        };

        for idx in path {
            node = &mut node.children[idx];
            node.visits += 1;
            node.total_value += if node.current_player == root_player {
                value
            } else {
                -value
            };
        }
    }
}

impl Agent for MCTS {
    fn select_action(
        &mut self,
        observation: &Observation,
        legal_actions: Vec<Action>,
        env: Option<&dyn Env>,
    ) -> Action {
        let env = env.expect("MCTS requires env");
        let mut root = MCTSNode::new(None, env.current_player(), env.legal_action());
        let root_player = root.current_player;

        for _ in 0..self.n_simulations {
            self.run_simulation(&mut root, env, root_player);
        }

        root.children
            .iter()
            .max_by_key(|c| c.visits)
            .map(|c| c.action.unwrap())
            .unwrap()
    }
}
