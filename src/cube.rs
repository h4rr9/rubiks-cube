use crate::{
    cubies::{NUM_CORNERS, NUM_EDGES},
    orientation::{CornerOrientation, EdgeOrientation},
    permutation::Permutation,
};

pub struct Cube {
    edge_orientation: EdgeOrientation,
    corner_orientation: CornerOrientation,
    edge_permutation: Permutation,
    corner_permutation: Permutation,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            edge_orientation: EdgeOrientation::new(),
            corner_orientation: CornerOrientation::new(),
            edge_permutation: Permutation::new(NUM_EDGES),
            corner_permutation: Permutation::new(NUM_CORNERS),
        }
    }
}
