use fixedbitset::FixedBitSet;

use super::faces::{Edge, EDGES, NUM_EDGES};

#[derive(Debug)]
pub struct EdgeOrientation {
    orientations: FixedBitSet,
}

impl EdgeOrientation {
    pub fn new() -> EdgeOrientation {
        EdgeOrientation {
            orientations: FixedBitSet::with_capacity(NUM_EDGES as usize),
        }
    }

    pub fn new_with_orientation(n: u16) -> EdgeOrientation {
        EdgeOrientation {
            orientations: FixedBitSet::with_capacity_and_blocks(NUM_EDGES as usize, vec![n as u32]),
        }
    }

    pub fn orientation_at_index(&self, idx: u8) -> u8 {
        self.orientations[idx as usize] as u8
    }

    pub fn edge_by_index(&self, idx: u8) -> &Edge {
        &EDGES[idx as usize]
    }

    pub fn add_one(&mut self, idx: u8) {
        self.orientations.toggle(idx as usize);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sets::faces::{EDGES, NUM_EDGES};

    #[test]
    fn edge_and_orientation_test() {
        let edge_set = EdgeOrientation::new();

        for i in 0..NUM_EDGES {
            let edge = *edge_set.edge_by_index(i);
            let edge_orientation = edge_set.orientation_at_index(i);

            assert_eq!(edge, EDGES[i as usize]);
            assert_eq!(edge_orientation, 0);
        }
    }

    #[test]
    fn first_orientation_test() {
        let edge_set = EdgeOrientation::new_with_orientation(0b111111111111);

        for i in 0..NUM_EDGES {
            let edge_orientation = edge_set.orientation_at_index(i);
            assert_eq!(edge_orientation, 1);
        }
    }

    #[test]
    fn add_one_test() {
        let mut edge_set = EdgeOrientation::new_with_orientation(0b111111111111);

        for i in 0..NUM_EDGES {
            let edge_orientation = edge_set.orientation_at_index(i);
            edge_set.add_one(i);
            assert_eq!((edge_orientation + 1) % 2, edge_set.orientation_at_index(i));
        }
    }
}
