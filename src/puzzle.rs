use crate::piece::{Joint, MAX_ROT};

// The puzzle definition for "One Tough Puzzle"
pub const JIGSAW: [[Joint; MAX_ROT]; 9] = [
    [Joint::SpadeTab, Joint::SpadeTab, Joint::HeartSlot, Joint::ClubSlot], // Piece 8
    [Joint::SpadeTab, Joint::DiamondTab, Joint::SpadeSlot, Joint::HeartSlot], // Piece 7
    [Joint::HeartTab, Joint::DiamondTab, Joint::DiamondSlot, Joint::HeartSlot], // Piece 6
    [Joint::DiamondTab, Joint::ClubTab, Joint::ClubSlot, Joint::DiamondSlot], // Piece 5
    [Joint::ClubTab, Joint::HeartTab, Joint::SpadeSlot, Joint::HeartSlot], // Piece 4
    [Joint::HeartTab, Joint::SpadeTab, Joint::SpadeSlot, Joint::ClubSlot], // Piece 3
    [Joint::HeartTab, Joint::DiamondTab, Joint::ClubSlot, Joint::ClubSlot], // Piece 2
    [Joint::ClubTab, Joint::HeartTab, Joint::DiamondSlot, Joint::ClubSlot], // Piece 1
    [Joint::SpadeTab, Joint::DiamondTab, Joint::HeartSlot, Joint::DiamondSlot], // Piece 0
];
