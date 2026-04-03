use crate::traits::{Agent, Env, EpisodeResult, Observation, RunStats};
use std::time::Instant;
use std::io::{self, Write};

pub struct Runner;

impl Runner {
    pub fn run_episode(env: &mut dyn Env, agent: &mut dyn Agent, eval_mode: bool) -> EpisodeResult {
        let mut observation = env.reset();
        let mut total_reward = 0.0;
        let mut num_steps = 0;
        let mut total_move_time_ms = 0.0;

        loop {
            let legal_actions = env.legal_action();
            let t0 = Instant::now();
            let action = agent.select_action(&observation, legal_actions, Some(&*env));
            total_move_time_ms = total_move_time_ms + t0.elapsed().as_secs_f64() * 1000.0;

            let (next_observation, reward, is_done) = env.step(action);
            total_reward += reward;
            num_steps += 1;
            observation = next_observation;

            if is_done {
                break;
            }
        }
        EpisodeResult {
            score: env.score(0),
            num_steps,
            move_time_ms: total_move_time_ms / num_steps as f64,
        }
    }

    pub fn run_n_episodes(
        env: &mut dyn Env,
        agent: &mut dyn Agent,
        eval_mode: bool,
        n: usize,
    ) -> RunStats {
        let mut scores = Vec::with_capacity(n);
        let mut steps = Vec::with_capacity(n);
        let mut times = Vec::with_capacity(n);

        for i in 0..n {
            print!("\r  Episode {}/{}", i, n);
            io::stdout().flush().unwrap();
            let res = Self::run_episode(env, agent, eval_mode);
            scores.push(res.score);
            steps.push(res.num_steps);
            times.push(res.move_time_ms);
        }

        print!("\r{}", " ".repeat(40));
        print!("\r");
        io::stdout().flush().unwrap();

        RunStats {
            mean_score: mean(&scores),
            mean_steps: mean_usize(&steps),
            mean_move_time_ms: mean_f64(&times),
            n_episodes: n,
        }
    }
}

fn mean(p0: &Vec<f32>) -> f32 {
    p0.iter().sum::<f32>() / p0.len() as f32
}

fn mean_f64(p0: &Vec<f64>) -> f64 {
    p0.iter().sum::<f64>() / p0.len() as f64
}

fn mean_usize(p0: &Vec<usize>) -> f32 {
    p0.iter().sum::<usize>() as f32 / p0.len() as f32
}
