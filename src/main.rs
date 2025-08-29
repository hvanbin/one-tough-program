mod tests;

use jigsaw_solver::{solve_puzzle, print_solution};

fn main() {
    if let Some(solution) = solve_puzzle() {
        print_solution(&solution);
        println!();
    } else {
        println!("No solution");
    }
}
