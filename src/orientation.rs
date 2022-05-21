use crate::cubies::{NUM_CORNERS, NUM_EDGES};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Orientation {
    orientations: Vec<u8>,
    n_orientation: u8,
}

impl Orientation {
    pub fn edge() -> Orientation {
        Self {
            orientations: vec![0; NUM_EDGES as usize],
            n_orientation: 2,
        }
    }

    pub fn corner() -> Orientation {
        Self {
            orientations: vec![0; NUM_CORNERS as usize],
            n_orientation: 3,
        }
    }

    pub fn orientation_at_index(&self, idx: u8) -> u8 {
        self.orientations[idx as usize] as u8
    }

    pub fn add_one(&mut self, idx: u8) {
        self.orientations[idx as usize] = match self.orientations[idx as usize] {
            0 => 1,
            1 => {
                if self.n_orientation == 2 {
                    0
                } else {
                    2
                }
            }
            2 => {
                if self.n_orientation == 2 {
                    1
                } else {
                    0
                }
            }
            _ => panic!("invalid orientation encountered. panicing!"),
        }
    }

    pub fn add_two(&mut self, idx: u8) {
        self.orientations[idx as usize] = match self.orientations[idx as usize] {
            0 => 2,
            1 => {
                if self.n_orientation == 2 {
                    1
                } else {
                    0
                }
            }
            2 => {
                if self.n_orientation == 2 {
                    0
                } else {
                    1
                }
            }
            _ => panic!("invalid orientation encountered. panicing!"),
        }
    }

    pub fn sum(&self) -> u8 {
        self.orientations.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Orientation;
    use crate::cubies::{Corner, Edge, CORNER_CUBIES, EDGE_CUBIES, NUM_CORNERS, NUM_EDGES};

    impl Orientation {
        pub fn edge_by_index(&self, idx: u8) -> &Edge {
            &EDGE_CUBIES[idx as usize]
        }

        pub fn corner_by_index(&self, idx: u8) -> &Corner {
            &CORNER_CUBIES[idx as usize]
        }

        pub fn new_with_orientation(o: Vec<u8>) -> Orientation {
            assert!(o.len() == NUM_EDGES as usize || o.len() == NUM_CORNERS as usize);
            Orientation {
                n_orientation: if o.len() == NUM_EDGES as usize { 2 } else { 3 },
                orientations: o,
            }
        }
    }

    #[test]
    fn edge_and_orientation_test() {
        let edge_set = Orientation::edge();

        for i in 0..NUM_EDGES {
            let edge = *edge_set.edge_by_index(i);
            let edge_orientation = edge_set.orientation_at_index(i);

            assert_eq!(edge, EDGE_CUBIES[i as usize]);
            assert_eq!(edge_orientation, 0);
        }
    }

    #[test]
    fn first_orientation_test() {
        let edge_set = Orientation::new_with_orientation(vec![1; NUM_EDGES as usize]);

        for i in 0..NUM_EDGES {
            let edge_orientation = edge_set.orientation_at_index(i);
            assert_eq!(edge_orientation, 1);
        }
    }

    #[test]
    fn add_one_test() {
        let mut edge_set = Orientation::new_with_orientation(vec![1; NUM_EDGES as usize]);

        for i in 0..NUM_EDGES {
            let edge_orientation = edge_set.orientation_at_index(i);
            edge_set.add_one(i);
            assert_eq!((edge_orientation + 1) % 2, edge_set.orientation_at_index(i));
        }
    }

    #[test]
    fn sum_test() {
        let edge_set = Orientation::new_with_orientation(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(6, edge_set.sum());
    }
}
