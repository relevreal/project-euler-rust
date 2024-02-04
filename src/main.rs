use std::time::Instant;
use clap::Parser;
use project_euler::*;

#[derive(Parser, Debug)]
struct Args {
    problem: usize,
}

fn main() {
    let args = Args::parse();

    let now = Instant::now();
    let solution = match args.problem {
        1 => problem_1::solve(),
        2 => problem_2::solve(),
        3 => problem_3::solve(),
        4 => problem_4::solve(),
        5 => problem_5::solve(),
        6 => problem_6::solve(),
        7 => problem_7::solve(),
        8 => problem_8::solve(),
        9 => problem_9::solve(),
        10 => problem_10::solve(),
        11 => problem_11::solve(),
        12 => problem_12::solve(),
        _ => panic!("Problem {} not found!", args.problem),
    };
    let elapsed = now.elapsed();

    println!("Problem {} solution: {}, solved in: {:.2?}", args.problem, solution, elapsed);
}
