use std::fmt::Display;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, PartialEq)]
pub enum Moves {
    L,  // Clockwise Left turn
    R,  // Clockwise Right turn
    F,  // Clockwise Front turn
    B,  // Clockwise Back turn
    U,  // Clockwise Up turn
    D,  // Clockwise Down turn
    L_, // Anti-Clockwise Left turn
    R_, // Anti-Clockwise Right turn
    F_, // Anti-Clockwise Front turn
    B_, // Anti-Clockwise Back turn
    U_, // Anti-Clockwise Up turn
    D_, // Anti-lockwise Down turn
    L2, // Half Left turn
    R2, // Half Right turn
    F2, // Half Front turn
    B2, // Half Back turn
    U2, // Half Up turn
    D2, // Half Down turn
}

impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Moves::L => write!(f, "L "),
            Moves::R => write!(f, "R "),
            Moves::F => write!(f, "F "),
            Moves::B => write!(f, "B "),
            Moves::U => write!(f, "U "),
            Moves::D => write!(f, "D "),
            Moves::L_ => write!(f, "L_ "),
            Moves::R_ => write!(f, "R_ "),
            Moves::F_ => write!(f, "F_ "),
            Moves::B_ => write!(f, "B_ "),
            Moves::U_ => write!(f, "U_ "),
            Moves::D_ => write!(f, "D_ "),
            Moves::L2 => write!(f, "L2 "),
            Moves::R2 => write!(f, "R2 "),
            Moves::F2 => write!(f, "F2 "),
            Moves::B2 => write!(f, "B2 "),
            Moves::U2 => write!(f, "U2 "),
            Moves::D2 => write!(f, "D2 "),
        }
    }
}

impl Distribution<Moves> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Moves {
        match rng.gen_range(0..18) {
            0 => Moves::L,
            1 => Moves::R,
            2 => Moves::F,
            3 => Moves::B,
            4 => Moves::U,
            5 => Moves::D,

            6 => Moves::L_,
            7 => Moves::R_,
            8 => Moves::F_,
            9 => Moves::B_,
            10 => Moves::U_,
            11 => Moves::D_,

            12 => Moves::L2,
            13 => Moves::R2,
            14 => Moves::F2,
            15 => Moves::B2,
            16 => Moves::U2,
            _ => Moves::D2,
        }
    }
}
