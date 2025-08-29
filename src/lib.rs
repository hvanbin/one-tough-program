//! # Jigsaw Puzzle Solver
//! 
//! A library for solving "One Tough Puzzle" - a 3x3 jigsaw puzzle where pieces have tabs and slots.
//! 
//! ## Example
//! 
//! ```rust
//! use jigsaw_solver::{solve_puzzle, print_solution};
//! 
//! let solution = solve_puzzle();
//! if let Some(sol) = solution {
//!     print_solution(&sol);
//! } else {
//!     println!("No solution found");
//! }
//! ```

pub mod piece;
pub mod solver;
pub mod puzzle;

pub use piece::{Piece, Side, Joint, fit_joint};
pub use solver::{solve, fit_piece, find_possibles, print_solution, MAX_WIDTH, MAX_POS};
pub use puzzle::JIGSAW;

/// Convenience function to solve the puzzle and return the solution
pub fn solve_puzzle() -> Option<[Option<Piece>; MAX_POS]> {
    let mut solution: [Option<Piece>; MAX_POS] = [None; MAX_POS];
    let used = 0u32;

    if solve(&mut solution, 0, used, &JIGSAW) {
        Some(solution)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle_convenience_function() {
        let solution = solve_puzzle();
        assert!(solution.is_some());
        
        if let Some(sol) = solution {
            // Verify all positions are filled
            for piece_opt in &sol {
                assert!(piece_opt.is_some());
            }
        }
    }
}
