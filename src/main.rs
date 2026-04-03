mod agents;
mod environments;
mod training;
mod traits;

use crate::agents::random::RandomAgent;
use crate::agents::random_rollout::RandomRolloutAgent;
use crate::environments::line_world::LineWorld;
use crate::training::runner::Runner;
use crate::traits::Env;
use burn::backend::Wgpu;
use burn::tensor::Tensor;
use crate::environments::grid_world::GridWorld;

type Backend = Wgpu;

fn lineworld_and_random_agent() {
    let mut env = LineWorld::new(7, 3, 50, false);
    let mut agent = RandomAgent;

    println!("--- LineWorld with RandomAgent ---");

    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut agent, true, n);
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
        let stats = Runner::run_n_episodes(&mut env, &mut agent, true, n);
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
        let stats = Runner::run_n_episodes(&mut env, &mut agent, true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms,
        )
    }
}

fn main() {
    // lineworld_and_random_agent();
    // gridworld_and_random_agent();
    gridworld_and_randomrollout_agent();
}
