mod agents;
mod environments;
mod training;
mod traits;
mod ui;

use crate::agents::mcts::MCTS;
use crate::agents::random::RandomAgent;
use crate::agents::random_rollout::RandomRolloutAgent;
use crate::agents::reinforce::ReinforceAgent;

use crate::environments::grid_world::GridWorld;
use crate::environments::line_world::LineWorld;
use crate::environments::pond::Pond;
use crate::environments::tictactoe::TicTacToe;

use crate::training::runner::Runner;

use crate::traits::Env;

use burn::backend::{Autodiff, Wgpu};
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
            "Episodes: {:>8} | Score moyen RandomAgent: {:.3} | Score moyen MCTS : {:.3}| Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms,
        )
    }
}

fn mcts_vs_mcts_tictactoe() {
    let mut env = TicTacToe::new();
    let mut agent = MCTS::new(200, (2.0f32).sqrt());
    let mut agent2 = MCTS::new(200, (2.0f32).sqrt());
    println!("--- MCTS with MCTS in TicTacToe ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score moyen MCTS1: {:.3} | Score moyen MCTS2 : {:.3}| Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms,
        )
    }
}

fn random_vs_random_pond() {
    let mut env = Pond::new();
    let mut agent = RandomAgent;
    let mut agent2 = RandomAgent;
    println!("--- Random vs Random in Pond ---");
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

fn mcts_vs_mcts_pond() {
    let mut env = Pond::new();
    let mut agent = MCTS::new(200, (2.0f32).sqrt());
    let mut agent2 = MCTS::new(200, (2.0f32).sqrt());
    println!("--- MCTS vs MCTS in Pond ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score moyen MCTS1: {:.3} | Score moyen MCTS2 : {:.3}| Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms,
        )
    }
}

fn reinforce_vs_random_tictactoes() {
    type B = Autodiff<Wgpu>;

    let device = Default::default();
    let mut env = TicTacToe::new();

    let mut agent = ReinforceAgent::<B>::new(
        env.observation_size(),
        env.num_actions(),
        64,
        0.99,
        device,
        1e-3,
    );
    let mut agent2 = RandomAgent;
    println!("--- REINFORCE vs RandomAgent on TicTacToe ---");

    let checkpoints = [100, 1_000, 10_000, 100_000];
    let mut previous = 0;

    for &n in &checkpoints {
        let remaining = n - previous;
        Runner::train_n_episodes(&mut env, &mut [&mut agent, &mut agent2], remaining);
        previous = n;

        agent.eval_mode = true;
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, 1_000);
        agent.eval_mode = false;

        println!(
            "Episodes: {:>8} | Score moyen REINFORCE: {:.3} | Score moyen Random : {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            n,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms
        );
    }
}
fn reinforce_vs_random_pond() {
    type B = Autodiff<Wgpu>;

    let device = Default::default();
    let mut env = Pond::new();

    let mut agent = ReinforceAgent::<B>::new(
        env.observation_size(),
        env.num_actions(),
        64,
        0.99,
        device,
        1e-3,
    );
    let mut agent2 = RandomAgent;
    println!("--- REINFORCE vs RandomAgent on Pond ---");

    let checkpoints = [100, 1_000, 10_000, 100_000];
    let mut previous = 0;

    for &n in &checkpoints {
        let remaining = n - previous;
        Runner::train_n_episodes(&mut env, &mut [&mut agent, &mut agent2], remaining);
        previous = n;

        agent.eval_mode = true;
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, 1_000);
        agent.eval_mode = false;

        println!(
            "Episodes: {:>8} | Score moyen REINFORCE: {:.3} | Score moyen Random : {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            n,
            stats.mean_score,
            stats.mean_score_player_2,
            stats.mean_steps,
            stats.mean_move_time_ms
        );
    }
}

fn main() {
    // lineworld_and_random_agent();
    // gridworld_and_random_agent();
    // gridworld_and_randomrollout_agent();
    // random_vs_random_tictactoe();
    // random_vs_mcts_tictactoe();
    // mcts_vs_mcts_tictactoe();
    // random_vs_random_pond()
    // mcts_vs_mcts_pond();
    // reinforce_vs_random_tictactoes();
    reinforce_vs_random_pond();
}
