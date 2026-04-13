mod agents;
mod environments;
mod training;
mod traits;

use crate::agents::random::RandomAgent;
use crate::agents::random_rollout::RandomRolloutAgent;
use crate::agents::mcts::MCTS;
use crate::environments::grid_world::GridWorld;
use crate::environments::line_world::LineWorld;
use crate::environments::tictactoe::TicTacToe;
use crate::training::runner::Runner;
use crate::traits::Env;
use burn::backend::Wgpu;
use burn::tensor::Tensor;

type Backend = Wgpu;

fn lineworld_and_random_agent() {
    let mut env = LineWorld::new(7, 3, 50, false);
    let mut agent = RandomAgent;

    println!("--- LineWorld with RandomAgent ---");

    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms,
        );
    }
}

fn gridworld_and_random_agent() {
    let mut env = GridWorld::new(5, 5, 50, false);
    let mut agent = RandomAgent;
    println!("--- GridWorld with RandomAgent ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms,
        )
    }
}

fn gridworld_and_randomrollout_agent() {
    let mut env = GridWorld::new(5, 5, 50, false);
    let mut agent = RandomRolloutAgent::new(50);
    println!("--- GridWorld with RandomRolloutAgent ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms,
        )
    }
}

fn random_vs_random_tictactoe() {
    let mut env = TicTacToe::new();
    let mut agent = RandomAgent;
    let mut agent2 = RandomAgent;
    println!("--- RandomAgent with RandomAgent in TicTacToe ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Score moyen player 2 : {:.3}| Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms,
        )
    }
}

fn random_vs_mcts_tictactoe() {
    let mut env = TicTacToe::new();
    let mut agent = RandomAgent;
    let mut agent2 = MCTS::new(200, (2.0f32).sqrt());
    println!("--- RandomAgent with MCTS in TicTacToe ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Score moyen player 2 : {:.3}| Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms,
        )
    }
}

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        // lineworld_and_random_agent();
        // gridworld_and_random_agent();
        // gridworld_and_randomrollout_agent();
        // random_vs_random_tictactoe();
        //random_vs_mcts_tictactoe();

        clear_background(RED);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await }
}


