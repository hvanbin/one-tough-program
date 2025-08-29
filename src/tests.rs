#[cfg(test)]
mod tests {
    use jigsaw_solver::piece::{Piece, Side, Joint, fit_joint, MAX_ROT};
    use jigsaw_solver::solver::{fit_piece, find_possibles, solve, MAX_POS, MAX_WIDTH};
    use jigsaw_solver::puzzle::JIGSAW;

    #[test]
    fn test_joint_fitting() {
        // Test that tabs fit into corresponding slots
        assert!(fit_joint(Joint::ClubTab, Joint::ClubSlot));
        assert!(fit_joint(Joint::ClubSlot, Joint::ClubTab));
        assert!(fit_joint(Joint::HeartTab, Joint::HeartSlot));
        assert!(fit_joint(Joint::HeartSlot, Joint::HeartTab));
        assert!(fit_joint(Joint::DiamondTab, Joint::DiamondSlot));
        assert!(fit_joint(Joint::DiamondSlot, Joint::DiamondTab));
        assert!(fit_joint(Joint::SpadeTab, Joint::SpadeSlot));
        assert!(fit_joint(Joint::SpadeSlot, Joint::SpadeTab));

        // Test that mismatched joints don't fit
        assert!(!fit_joint(Joint::ClubTab, Joint::HeartSlot));
        assert!(!fit_joint(Joint::SpadeSlot, Joint::DiamondTab));
        assert!(!fit_joint(Joint::HeartTab, Joint::ClubSlot));
    }

    #[test]
    fn test_piece_creation() {
        let piece = Piece::new(0, 0, &JIGSAW);
        assert_eq!(piece.index, 0);
        assert_eq!(piece.rotation, 0);
        assert_eq!(piece.edges, JIGSAW[0]);
    }

    #[test]
    fn test_piece_rotation() {
        // Test piece 8 (index 0 in JIGSAW array): [SpadeTab, SpadeTab, HeartSlot, ClubSlot]
        let piece = Piece::new(0, 1, &JIGSAW); // Rotate piece 0 by 1 position
        
        // With rotation 1, the joints should be shifted
        // Original piece 0: [SpadeTab, SpadeTab, HeartSlot, ClubSlot]
        // Side::Top (0) with rotation 1 should get edge[(0-1+4)%4] = edge[3] = ClubSlot
        assert_eq!(piece.get_joint(Side::Top), Joint::ClubSlot);
        assert_eq!(piece.get_joint(Side::Right), Joint::SpadeTab);
        assert_eq!(piece.get_joint(Side::Bottom), Joint::SpadeTab);
        assert_eq!(piece.get_joint(Side::Left), Joint::HeartSlot);
    }

    #[test]
    fn test_fit_piece_corner() {
        let solution: [Option<Piece>; MAX_POS] = [None; MAX_POS];
        
        // Test fitting a piece in the top-left corner (position 0)
        // Any piece should fit in the corner since there are no constraints
        let piece = Piece::new(0, 0, &JIGSAW);
        assert!(fit_piece(&solution, &piece, 0));
    }

    #[test]
    fn test_fit_piece_with_constraints() {
        let mut solution: [Option<Piece>; MAX_POS] = [None; MAX_POS];
        
        // Place a piece at position 0
        let corner_piece = Piece::new(0, 0, &JIGSAW); // SpadeTab, SpadeTab, HeartSlot, ClubSlot
        solution[0] = Some(corner_piece);
        
        // Try to place a piece at position 1 (to the right)
        // The left edge of the new piece must fit with the right edge of corner_piece
        // corner_piece right edge is SpadeTab, so we need SpadeSlot on the left
        
        // Create a test piece that should fit
        let mut test_piece = Piece::new(1, 0, &JIGSAW);
        // We need to find a rotation where the left edge is SpadeSlot
        for rotation in 0..MAX_ROT {
            test_piece.rotation = rotation;
            if test_piece.get_joint(Side::Left) == Joint::SpadeSlot {
                assert!(fit_piece(&solution, &test_piece, 1));
                break;
            }
        }
    }

    #[test]
    fn test_solver_finds_solution() {
        let mut solution: [Option<Piece>; MAX_POS] = [None; MAX_POS];
        let used = 0u32;
        
        // The solver should find a valid solution
        assert!(solve(&mut solution, 0, used, &JIGSAW));
        
        // Verify that all positions are filled
        for piece_opt in &solution {
            assert!(piece_opt.is_some());
        }
        
        // Verify that all pieces are used exactly once
        let mut used_indices = vec![false; MAX_POS];
        for piece_opt in &solution {
            if let Some(piece) = piece_opt {
                assert!(!used_indices[piece.index], "Piece {} used more than once", piece.index);
                used_indices[piece.index] = true;
            }
        }
        
        // Verify that the solution is valid by checking all adjacent pieces
        for pos in 0..MAX_POS {
            if let Some(piece) = &solution[pos] {
                // Check right neighbor
                if pos % MAX_WIDTH != MAX_WIDTH - 1 {
                    if let Some(right_piece) = &solution[pos + 1] {
                        assert!(fit_joint(
                            piece.get_joint(Side::Right),
                            right_piece.get_joint(Side::Left)
                        ));
                    }
                }
                
                // Check bottom neighbor
                if pos + MAX_WIDTH < MAX_POS {
                    if let Some(bottom_piece) = &solution[pos + MAX_WIDTH] {
                        assert!(fit_joint(
                            piece.get_joint(Side::Bottom),
                            bottom_piece.get_joint(Side::Top)
                        ));
                    }
                }
            }
        }
    }

    #[test]
    fn test_find_possibles() {
        let solution: [Option<Piece>; MAX_POS] = [None; MAX_POS];
        let used = 0u32;
        
        // At position 0 with no constraints, we should find many possibilities
        let possibles = find_possibles(&solution, 0, used, &JIGSAW);
        assert!(!possibles.is_empty());
        
        // Each piece can have up to 4 rotations, so maximum is MAX_POS * MAX_ROT
        assert!(possibles.len() <= MAX_POS * MAX_ROT);
    }

    #[test]
    fn test_used_pieces_tracking() {
        let solution: [Option<Piece>; MAX_POS] = [None; MAX_POS];
        let used = 0b101; // Pieces 0 and 2 are used
        
        let possibles = find_possibles(&solution, 0, used, &JIGSAW);
        
        // Verify that pieces 0 and 2 are not in the possibles
        for piece in &possibles {
            assert!(piece.index != 0 && piece.index != 2);
        }
    }
}
