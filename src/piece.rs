use std::fmt;

// Constants
pub const MAX_ROT: usize = 4;

// Enums
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Joint {
    ClubSlot = -4,
    SpadeSlot = -3,
    DiamondSlot = -2,
    HeartSlot = -1,
    ClubTab = 0,
    HeartTab = 1,
    DiamondTab = 2,
    SpadeTab = 3,
}

// Struct
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub edges: [Joint; MAX_ROT],
    pub rotation: usize,
    pub index: usize,
}

impl fmt::Display for Joint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Joint::ClubSlot => "CS",
            Joint::SpadeSlot => "SS",
            Joint::DiamondSlot => "DS",
            Joint::HeartSlot => "HS",
            Joint::ClubTab => "CT",
            Joint::HeartTab => "HT",
            Joint::DiamondTab => "DT",
            Joint::SpadeTab => "ST",
        };
        write!(f, "{}", s)
    }
}

impl Piece {
    pub fn new(index: usize, rotation: usize, jigsaw: &[[Joint; MAX_ROT]]) -> Self {
        Piece {
            edges: jigsaw[index],
            rotation,
            index,
        }
    }

    pub fn get_joint(&self, side: Side) -> Joint {
        let a = (side as i32 - self.rotation as i32).rem_euclid(MAX_ROT as i32) as usize;
        self.edges[a]
    }

    pub fn print(&self) {
        print!(
            "{},{},{},{}({})",
            self.edges[0], self.edges[1], self.edges[2], self.edges[3], self.rotation
        );
    }
}

pub fn fit_joint(j1: Joint, j2: Joint) -> bool {
    matches!(
        (j1, j2),
        (Joint::ClubSlot, Joint::ClubTab)
            | (Joint::ClubTab, Joint::ClubSlot)
            | (Joint::SpadeSlot, Joint::SpadeTab)
            | (Joint::SpadeTab, Joint::SpadeSlot)
            | (Joint::DiamondSlot, Joint::DiamondTab)
            | (Joint::DiamondTab, Joint::DiamondSlot)
            | (Joint::HeartSlot, Joint::HeartTab)
            | (Joint::HeartTab, Joint::HeartSlot)
    )
}
