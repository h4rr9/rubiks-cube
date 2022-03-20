use super::faces::{Corner, CORNERS, NUM_CORNERS};

#[derive(Debug)]
pub struct CornerOrientation {
    orientations: u16,
}

impl CornerOrientation {
    pub fn new() -> CornerOrientation {
        CornerOrientation { orientations: 0 }
    }

    pub fn new_with_orientation(n: u16) -> CornerOrientation {
        CornerOrientation { orientations: n }
    }

    pub fn orientation_at_index(&self, idx: u8) -> u8 {
        if idx > NUM_CORNERS {
            panic!(
                "corner index out of bounds, expected value less than 8, got {}",
                idx
            );
        }
        ((self.orientations >> (idx << 1)) as u8) & 3u8
    }

    pub fn corner_by_index(&self, idx: u8) -> &Corner {
        if idx > NUM_CORNERS {
            panic!(
                "corner index out of bounds, expected value less than 8, got {}",
                idx
            );
        }
        &CORNERS[idx as usize]
    }

    pub fn add_one(&mut self, idx: u8) {
        let new_value = match self.orientation_at_index(idx) {
            0 => 1u16,
            1 => 2u16,
            2 => 0u16,
            other_value => panic!("invalid orientation encountered {}", other_value),
        };
        let new_value = new_value << (idx << 1);
        let bitmask = 3 << (idx << 1);

        self.orientations = (self.orientations & !bitmask) | new_value;
    }

    pub fn add_two(&mut self, idx: u8) {
        let new_value = match self.orientation_at_index(idx) {
            0 => 2u16,
            1 => 0u16,
            2 => 1u16,
            other_value => panic!("invalid orientation encountered {}", other_value),
        };
        let new_value = new_value << (idx << 1);
        let bitmask = 3 << (idx << 1);

        self.orientations = (self.orientations & !bitmask) | new_value;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sets::faces::{CORNERS, NUM_CORNERS};

    #[test]
    fn corner_and_orientation_test() {
        let corner_set = CornerOrientation::new();

        for i in 0..NUM_CORNERS {
            let corner = *corner_set.corner_by_index(i);
            let corner_orientation = corner_set.orientation_at_index(i);

            assert_eq!(corner, CORNERS[i as usize]);
            assert_eq!(corner_orientation, 0);
        }
    }

    #[test]
    fn first_orientation_test() {
        let corner_set = CornerOrientation::new_with_orientation(0b1010101010101010);

        for i in 0..NUM_CORNERS {
            let corner_orientation = corner_set.orientation_at_index(i);
            assert_eq!(corner_orientation, 2);
        }
    }

    #[test]
    fn second_orientation_test() {
        let corner_set = CornerOrientation::new_with_orientation(0b0101010101010101);

        for i in 0..NUM_CORNERS {
            let corner_orientation = corner_set.orientation_at_index(i);
            assert_eq!(corner_orientation, 1);
        }
    }

    #[test]
    fn add_one_test() {
        let mut corner_set = CornerOrientation::new_with_orientation(0b0101010101010101);

        for i in 0..NUM_CORNERS {
            let corner_orientation = corner_set.orientation_at_index(i);
            corner_set.add_one(i);
            assert_eq!(
                (corner_orientation + 1) % 3,
                corner_set.orientation_at_index(i)
            );
        }
    }

    #[test]
    fn add_two_test() {
        let mut corner_set = CornerOrientation::new_with_orientation(0b0101010101010101);

        for i in 0..NUM_CORNERS {
            let corner_orientation = corner_set.orientation_at_index(i);
            corner_set.add_two(i);
            assert_eq!(
                (corner_orientation + 2) % 3,
                corner_set.orientation_at_index(i)
            );
        }
    }
}
