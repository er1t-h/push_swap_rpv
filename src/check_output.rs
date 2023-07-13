use std::fmt::Display;

use crate::stack::Stack;

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

pub fn check(mut stack_a: Stack, output: &str) -> Result<usize, ExecError> {
    let mut stack_b: Stack = Stack::with_capacity(stack_a.len());
    let mut nb_line = 0;
    for line in output.lines().map(str::trim) {
        nb_line += 1;
        match line {
            "sa" => stack_a.swap(),
            "sb" => stack_b.swap(),
            "ss" => {
                stack_a.swap();
                stack_b.swap();
            }
            "ra" => stack_a.rotate(),
            "rb" => stack_b.rotate(),
            "rr" => {
                stack_a.rotate();
                stack_b.rotate();
            }
            "rra" => stack_a.reverse_rotate(),
            "rrb" => stack_b.reverse_rotate(),
            "rrr" => {
                stack_a.reverse_rotate();
                stack_b.reverse_rotate();
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
