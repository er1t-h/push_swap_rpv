mod cli;
mod stack;

use std::fmt::Display;
use std::time::Instant;
use std::{ffi::OsString, process::Command};

use clap::Parser;
use rayon::prelude::*;
use stack::Stack;

use crate::cli::CliArgs;
use rand::seq::SliceRandom;

pub enum ExecError {
    BadCommand(String),
    StackBNotEmpty,
    NotSorted,
}

impl Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadCommand(x) => write!(f, "unknown command: {x}"),
            Self::StackBNotEmpty => write!(f, "stack b is not empty"),
            Self::NotSorted => write!(f, "stack a is not sorted"),
        }
    }
}

fn check_output(numbers: &[i32], output: &str) -> Result<usize, ExecError> {
    let mut stack_a: Stack = Stack::from_iter(numbers.iter().copied());
    let mut stack_b: Stack = Stack::with_capacity(numbers.len());
    let mut nb_line = 0;
    for line in output.lines().map(|x| x.trim()) {
        nb_line += 1;
        match line {
            "sa" => stack_a.swap(),
            "sb" => stack_b.swap(),
            "ss" => {
                stack_a.swap();
                stack_b.swap()
            }
            "ra" => stack_a.rotate(),
            "rb" => stack_b.rotate(),
            "rr" => {
                stack_a.rotate();
                stack_b.rotate()
            }
            "rra" => stack_a.reverse_rotate(),
            "rrb" => stack_b.reverse_rotate(),
            "rrr" => {
                stack_a.reverse_rotate();
                stack_b.reverse_rotate()
            }
            "pa" => stack_a.receive_push_from_other(&mut stack_b),
            "pb" => stack_b.receive_push_from_other(&mut stack_a),
            command => return Err(ExecError::BadCommand(command.to_string())),
        }
    }
    if !stack_b.is_empty() {
        Err(ExecError::StackBNotEmpty)
    } else if !stack_a.is_sorted() {
        Err(ExecError::NotSorted)
    } else {
        Ok(nb_line)
    }
}

fn main() {
    let args = CliArgs::parse();
    if args.repeat_times == 0 {
        return;
    }
    let time = Instant::now();
    let mut all_moves: Vec<usize> = (0..args.repeat_times)
        .into_par_iter()
        .map_init(rand::thread_rng, |mut rng, index| {
            let hi_bound = args.number_in_stack / 2;
            let lo_bound = -(args.number_in_stack / 2 + args.number_in_stack % 2);
            let mut ps_args: Vec<i32> = (lo_bound..hi_bound).collect();
            ps_args.shuffle(&mut rng);
            let output = Command::new(args.path.as_path())
                .args(ps_args.iter().map(|x| OsString::from(x.to_string())))
                .output()
                .expect("Couldn't run push_swap");
            let output = String::from_utf8(output.stdout).expect("Non UTF-8 push_swap return");
            let has_worked = check_output(&ps_args, &output);
            match has_worked {
                Ok(size) => size,
                Err(e) => {
                    eprintln!("Error during run {index}: {e}");
                    panic!()
                }
            }
        })
        .collect();
    println!("Total time: {:?}", time.elapsed());
    all_moves.sort();
    println!("Best case: {}", all_moves.first().unwrap());
    println!("Worst case: {}", all_moves.last().unwrap());
    println!(
        "Average case: {}",
        all_moves.iter().sum::<usize>() as f64 / args.repeat_times as f64
    );
    let median = if all_moves.len() % 2 == 1 {
        all_moves[all_moves.len() / 2]
    } else {
        (all_moves[all_moves.len() / 2] + all_moves[all_moves.len() / 2 + 1]) / 2
    };
    println!("Median case: {}", median);
}
