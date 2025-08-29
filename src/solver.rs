use crate::piece::{Piece, Side, Joint, fit_joint, MAX_ROT};

// Constants
pub const MAX_WIDTH: usize = 3;
pub const MAX_POS: usize = 9;

pub fn fit_piece(solution: &[Option<Piece>], piece: &Piece, pos: usize) -> bool {
    // Check top
    let fit_top = if pos < MAX_WIDTH {
        true // No piece above
    } else {
        if let Some(ref top_piece) = solution[pos - MAX_WIDTH] {
            fit_joint(piece.get_joint(Side::Top), top_piece.get_joint(Side::Bottom))
        } else {
            false
        }
    };

    // Check left
    let fit_left = if pos % MAX_WIDTH == 0 {
        true // No piece to the left
    } else {
        if let Some(ref left_piece) = solution[pos - 1] {
            fit_joint(piece.get_joint(Side::Left), left_piece.get_joint(Side::Right))
        } else {
            false
        }
    };

    fit_top && fit_left
}

pub fn find_possibles(
    solution: &[Option<Piece>], 
    pos: usize, 
    used: u32, 
    jigsaw: &[[Joint; MAX_ROT]]
) -> Vec<Piece> {
    let mut matches = Vec::new();

    // For every possible piece
    for i in 0..MAX_POS {
        // Check if piece is not used
        if (used & (1 << i)) == 0 {
            // Try all rotations
            for rot in 0..MAX_ROT {
                let piece = Piece::new(i, rot, jigsaw);
                if fit_piece(solution, &piece, pos) {
                    matches.push(piece);
                }
            }
        }
    }

    matches
}

pub fn solve(
    solution: &mut [Option<Piece>], 
    pos: usize, 
    used: u32, 
    jigsaw: &[[Joint; MAX_ROT]]
) -> bool {
    if pos >= MAX_POS {
        return true;
    }

    let possibles = find_possibles(solution, pos, used, jigsaw);

    for piece in possibles {
        solution[pos] = Some(piece);
        let new_used = used | (1 << piece.index);

        if solve(solution, pos + 1, new_used, jigsaw) {
            return true;
        }

        solution[pos] = None;
    }

    false
}

pub fn print_solution(solution: &[Option<Piece>]) {
    for (i, piece_opt) in solution.iter().enumerate() {
        if i > 0 && i % MAX_WIDTH == 0 {
            if i != MAX_POS - 1 {
                print!(",");
            }
            println!();
        }
        if let Some(piece) = piece_opt {
            piece.print();
        }
    }
}
