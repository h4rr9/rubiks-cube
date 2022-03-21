use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
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
