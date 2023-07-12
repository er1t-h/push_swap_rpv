use std::path::PathBuf;

use clap::Parser;

fn get_default_exec_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("push_swap");
    path
}

#[derive(Parser)]
pub struct CliArgs {
    #[arg(long, default_value=get_default_exec_path().into_os_string())]
    /// The path to the `push_swap` exec. Defaults to `../push_swap`.
    pub path: PathBuf,
    #[arg(long, default_value_t = 500)]
    /// How many numbers will be given to `push_swap`.
    pub number_in_stack: i32,
    #[arg(long, default_value_t = 100)]
    /// How many times are we going to launch `push_swap`.
    pub repeat_times: usize,
}
