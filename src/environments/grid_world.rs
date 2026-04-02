pub struct GridWorld {
    /// Width of the grid
    width: usize,

    /// Height of the grid
    height: usize,

    
    agent_pos : (usize, usize),
    current_step : usize,
    game_over : bool,
    final_score : f64,
}
