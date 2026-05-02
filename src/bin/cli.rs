use DeepLearning::agents::mcts::MCTS;
use DeepLearning::agents::random::RandomAgent;
use DeepLearning::agents::random_rollout::RandomRolloutAgent;
use DeepLearning::environments::grid_world::GridWorld;
use DeepLearning::environments::line_world::LineWorld;
use DeepLearning::environments::tictactoe::TicTacToe;
use DeepLearning::training::runner::Runner;
use DeepLearning::environments::pond::Pond;

use std::io::{self, Write};

fn lineworld_and_random_agent() {
    let mut env = LineWorld::new(7, 3, 50, false);
    let mut agent = RandomAgent;

    println!("--- LineWorld with RandomAgent ---");

    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
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
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
    }
}

fn gridworld_and_randomrollout_agent() {
    let mut env = GridWorld::new(5, 5, 50, false);
    let mut agent = RandomRolloutAgent::new(50);
    println!("--- GridWorld with RandomRolloutAgent ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent], true, n);
        println!(
            "Episodes: {:>8} | Score moyen: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
    }
}

fn random_vs_random_tictactoe() {
    let mut env = TicTacToe::new();
    let mut agent = RandomAgent;
    let mut agent2 = RandomAgent;
    println!("--- RandomAgent vs RandomAgent in TicTacToe ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score P1: {:.3} | Score P2: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_score_player_2, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
    }
}

fn random_vs_mcts_tictactoe() {
    let mut env = TicTacToe::new();
    let mut agent = RandomAgent;
    let mut agent2 = MCTS::new(200, (2.0f32).sqrt());
    println!("--- RandomAgent vs MCTS in TicTacToe ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score Random: {:.3} | Score MCTS: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_score_player_2, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
    }
}

fn mcts_vs_mcts_tictactoe() {
    let mut env = TicTacToe::new();
    let mut agent = MCTS::new(200, (2.0f32).sqrt());
    let mut agent2 = MCTS::new(200, (2.0f32).sqrt());
    println!("--- MCTS vs MCTS in TicTacToe ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score MCTS1: {:.3} | Score MCTS2: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_score_player_2, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
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
            "Episodes: {:>8} | Score P1: {:.3} | Score P2: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_score_player_2, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
    }
}

fn randomRollout_vs_randomRollout_pond() {
    let mut env = Pond::new();
    let mut agent = RandomRolloutAgent::new(50);;
    let mut agent2 = RandomRolloutAgent::new(50);;
    println!("--- RandomRollout vs RandomRollout in Pond ---");
    for n in [1_000, 10_000, 100_000, 1_000_000] {
        let stats = Runner::run_n_episodes(&mut env, &mut [&mut agent, &mut agent2], true, n);
        println!(
            "Episodes: {:>8} | Score P1: {:.3} | Score P2: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_score_player_2, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
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
            "Episodes: {:>8} | Score MCTS1: {:.3} | Score MCTS2: {:.3} | Steps moyen: {:.1} | Temps/coup: {:.3}ms | {:.0} games/s",
            stats.n_episodes, stats.mean_score, stats.mean_score_player_2, stats.mean_steps, stats.mean_move_time_ms, stats.games_per_sec,
        );
    }
}

fn main() {
    loop {
        println!("\n========== Menu ==========");
        println!("1. LineWorld + RandomAgent");
        println!("2. GridWorld + RandomAgent");
        println!("3. GridWorld + RandomRolloutAgent");
        println!("4. Random vs Random (TicTacToe)");
        println!("5. Random vs MCTS (TicTacToe)");
        println!("6. MCTS vs MCTS (TicTacToe)");
        println!("7. Random vs Random (Pond)");
        println!("8. RandomRollout vs RandomRollout (Pond)");
        println!("9. MCTS vs MCTS (Pond)");
        println!("0. Quitter");
        println!("==========================");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => lineworld_and_random_agent(),
            "2" => gridworld_and_random_agent(),
            "3" => gridworld_and_randomrollout_agent(),
            "4" => random_vs_random_tictactoe(),
            "5" => random_vs_mcts_tictactoe(),
            "6" => mcts_vs_mcts_tictactoe(),
            "7" => random_vs_random_pond(),
            "8" => randomRollout_vs_randomRollout_pond(),
            "9" => mcts_vs_mcts_pond(),
            "0" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide, réessayez."),
        }
    }
}