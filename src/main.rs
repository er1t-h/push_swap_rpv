#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::module_name_repetitions)]

mod check_output;
mod cli;
mod get_threshold_values;
mod stack;

use ansi_term::Color;
use check_output as output;

use std::time::Instant;
use std::{ffi::OsString, process::Command};

use clap::Parser;
use rayon::prelude::*;
use stack::Stack;

use crate::cli::CliArgs;
use crate::get_threshold_values::get_threshold_values;

const COLORS: [Color; 6] = [
    Color::RGB(97, 5, 255),
    Color::RGB(6, 186, 21),
    Color::RGB(83, 255, 64),
    Color::RGB(224, 242, 24),
    Color::RGB(230, 135, 11),
    Color::RGB(230, 11, 11),
];

fn print_correction_criteria(moves: &[usize], thresholds: &[usize], repeat_times: usize) {
    println!();
    println!("Correction Criterias:");
    let thresholds_values = get_threshold_values(moves, thresholds);
    let last_value = thresholds_values[thresholds.len()];
    for (color, (threshold, value)) in COLORS.iter().zip(thresholds.iter().zip(thresholds_values)) {
        let to_print = format!(
            "Less than {}: {} ({:.2}%)",
            threshold,
            value,
            value as f64 / repeat_times as f64 * 100.
        );
        println!("{}", color.paint(to_print));
    }
    let to_print = format!(
        "No points: {} ({:.2}%)",
        last_value,
        last_value as f64 / repeat_times as f64 * 100.
    );
    println!("{}", COLORS.last().unwrap().paint(to_print));
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
            let mut ps_args: Stack = (lo_bound..hi_bound).map(|x| x * 100).collect();
            ps_args.shuffle(&mut rng);
            let output = Command::new(args.path.as_path())
                .args(ps_args.iter().map(|x| OsString::from(x.to_string())))
                .output()
                .expect("Couldn't run push_swap");
            let output = String::from_utf8(output.stdout).expect("Non UTF-8 push_swap return");
            let has_worked = output::check(ps_args, &output);
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
    all_moves.sort_unstable();
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
    println!("Median case: {median}");

    match args.number_in_stack {
        // If you wish to add more thresholds (replace what's between ``):
        //
        // `number_in_stack_required` => {
        //     const THRESHOLDS: [usize; `number_of_thresholds`] = [`your_thresholds`];
        //     print_correction_criteria(&all_moves, &THRESHOLDS, args.repeat_times);
        // }
        100 => {
            const THRESHOLDS: [usize; 5] = [700, 900, 1_100, 1_300, 1_500];
            print_correction_criteria(&all_moves, &THRESHOLDS, args.repeat_times);
        }
        500 => {
            const THRESHOLDS: [usize; 5] = [5_500, 7_000, 8_500, 10_000, 11_500];
            print_correction_criteria(&all_moves, &THRESHOLDS, args.repeat_times);
        }
        _ => (),
    }
}
