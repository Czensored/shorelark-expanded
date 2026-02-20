use lib_simulation::Simulation;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;

#[derive(Debug, Clone)]
struct Config {
    runs: u32,
    generations: u32,
    seed: u64,
    out: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            runs: 50,
            generations: 100,
            seed: 42,
            out: "results.csv".to_string(),
        }
    }
}

fn print_help(program: &str) {
    eprintln!(
        "Usage: {program} [--runs N] [--gens N] [--seed N] [--out PATH]\n\
         Defaults: --runs 50 --gens 100 --seed 42 --out results.csv"
    );
}

fn parse_u32_flag(flag: &str, value: Option<String>) -> u32 {
    match value.and_then(|v| v.parse::<u32>().ok()) {
        Some(n) if n > 0 => n,
        _ => {
            eprintln!("Invalid value for {flag}. Expected a positive integer.");
            process::exit(2);
        }
    }
}

fn parse_u64_flag(flag: &str, value: Option<String>) -> u64 {
    match value.and_then(|v| v.parse::<u64>().ok()) {
        Some(n) => n,
        None => {
            eprintln!("Invalid value for {flag}. Expected an unsigned integer.");
            process::exit(2);
        }
    }
}

fn parse_args() -> Config {
    let mut cfg = Config::default();
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "simulation-batch".to_string());

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--runs" => cfg.runs = parse_u32_flag("--runs", args.next()),
            "--gens" | "--generations" => {
                cfg.generations = parse_u32_flag("--gens/--generations", args.next())
            }
            "--seed" => cfg.seed = parse_u64_flag("--seed", args.next()),
            "--out" => match args.next() {
                Some(path) if !path.is_empty() => cfg.out = path,
                _ => {
                    eprintln!("Invalid value for --out. Expected a non-empty path.");
                    process::exit(2);
                }
            },
            "--help" | "-h" => {
                print_help(&program);
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown flag: {arg}");
                print_help(&program);
                process::exit(2);
            }
        }
    }

    cfg
}

fn run_batch(cfg: &Config) -> std::io::Result<()> {
    let file = File::create(&cfg.out)?;
    let mut out = BufWriter::new(file);

    writeln!(
        out,
        "run,generation,prey_min_fitness,prey_max_fitness,prey_avg_fitness,prey_dead,predator_min_fitness,predator_max_fitness,predator_avg_fitness"
    )?;

    let per_run_lines: Vec<String> = (0..cfg.runs)
        .into_par_iter()
        .map(|run| {
            let run_seed = cfg.seed.wrapping_add(run as u64);
            let mut rng = ChaCha8Rng::seed_from_u64(run_seed);
            let mut sim = Simulation::random(&mut rng);

            let mut lines = String::with_capacity((cfg.generations as usize) * 120);
            for _ in 0..cfg.generations {
                let stats = sim.fast_forward(&mut rng);
                writeln!(
                    lines,
                    "{},{},{:.6},{:.6},{:.6},{},{:.6},{:.6},{:.6}",
                    run,
                    stats.generation,
                    stats.prey_ga.min_fitness,
                    stats.prey_ga.max_fitness,
                    stats.prey_ga.avg_fitness,
                    stats.num_dead_prey,
                    stats.predator_ga.min_fitness,
                    stats.predator_ga.max_fitness,
                    stats.predator_ga.avg_fitness,
                )
                .expect("writing CSV row into String should not fail");
            }

            lines
        })
        .collect();

    for lines in per_run_lines {
        out.write_all(lines.as_bytes())?;
    }

    out.flush()
}

fn main() {
    let cfg = parse_args();
    if let Err(err) = run_batch(&cfg) {
        eprintln!("Failed to run batch simulation: {err}");
        process::exit(1);
    }
    eprintln!(
        "Wrote {} runs x {} generations to {}",
        cfg.runs, cfg.generations, cfg.out
    );
}
