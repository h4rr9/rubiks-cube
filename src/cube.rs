use crate::orientation::{CornerOrientation, EdgeOrientation};

pub struct Cube {
    edge_orientation: EdgeOrientation,
    corner_orientation: CornerOrientation,
    edge_permutation: Vec<u8>,
    corner_permutation: Vec<u8>
}
