mod sets;

#[derive(Debug)]
struct RubiksCube {
    edge_position: Vec<u8>,
    corner_position: Vec<u8>,
    edge_orientation: sets::EdgeOrientation,
    corner_orientation: sets::CornerOrientation,
}
