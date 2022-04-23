use std::fmt::Display;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, PartialEq)]
pub enum Turn {
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

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::L => write!(f, "L "),
            Turn::R => write!(f, "R "),
            Turn::F => write!(f, "F "),
            Turn::B => write!(f, "B "),
            Turn::U => write!(f, "U "),
            Turn::D => write!(f, "D "),
            Turn::L_ => write!(f, "L_ "),
            Turn::R_ => write!(f, "R_ "),
            Turn::F_ => write!(f, "F_ "),
            Turn::B_ => write!(f, "B_ "),
            Turn::U_ => write!(f, "U_ "),
            Turn::D_ => write!(f, "D_ "),
            Turn::L2 => write!(f, "L2 "),
            Turn::R2 => write!(f, "R2 "),
            Turn::F2 => write!(f, "F2 "),
            Turn::B2 => write!(f, "B2 "),
            Turn::U2 => write!(f, "U2 "),
            Turn::D2 => write!(f, "D2 "),
        }
    }
}

impl Distribution<Turn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turn {
        match rng.gen_range(0..18) {
            0 => Turn::L,
            1 => Turn::R,
            2 => Turn::F,
            3 => Turn::B,
            4 => Turn::U,
            5 => Turn::D,

            6 => Turn::L_,
            7 => Turn::R_,
            8 => Turn::F_,
            9 => Turn::B_,
            10 => Turn::U_,
            11 => Turn::D_,

            12 => Turn::L2,
            13 => Turn::R2,
            14 => Turn::F2,
            15 => Turn::B2,
            16 => Turn::U2,
            _ => Turn::D2,
        }
    }
}
