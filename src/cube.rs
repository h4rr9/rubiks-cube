use std::{char, fmt::Display};

use crate::{
    cubies::*,
    moves::Moves,
    orientation::{CornerOrientation, EdgeOrientation},
    permutation::Permutation,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
    // TODO: Implement this
    /* pub fn from_array(cube_arr: &[&[&[char; 3]; 3]; 6]) -> Cube {
        let edge_permutation = Permutation::new(NUM_EDGES);

        let edge_cubicles_indices = vec![(
            (Faces::Green, 0, 0),
            (Faces::Red, 0, 0),
            (Faces::Yellow, 0, 2),
        ), ((), (), ()];
    } */

    pub fn scramble(num_turns: u32) -> Cube {
        let mut cube = Cube::new();

        for _ in 0..num_turns {
            cube.turn(rand::random::<Moves>());
        }

        assert_eq!(
            cube.edge_permutation.parity(),
            cube.corner_permutation.parity(),
        );
        cube
    }

    pub fn turn(&mut self, m: Moves) {
        let ((a, b, c, d), (w, x, y, z)) = match m {
            Moves::L | Moves::L_ | Moves::L2 => (L_EDGE_CUBICLES, L_CORNER_CUBICLES),
            Moves::R | Moves::R_ | Moves::R2 => (R_EDGE_CUBICLES, R_CORNER_CUBICLES),
            Moves::F | Moves::F_ | Moves::F2 => (F_EDGE_CUBICLES, F_CORNER_CUBICLES),
            Moves::B | Moves::B_ | Moves::B2 => (B_EDGE_CUBICLES, B_CORNER_CUBICLES),
            Moves::U | Moves::U_ | Moves::U2 => (U_EDGE_CUBICLES, U_CORNER_CUBICLES),
            Moves::D | Moves::D_ | Moves::D2 => (D_EDGE_CUBICLES, D_CORNER_CUBICLES),
        };

        let cubie_a = self.edge_permutation.index(a);
        let cubie_b = self.edge_permutation.index(b);
        let cubie_c = self.edge_permutation.index(c);
        let cubie_d = self.edge_permutation.index(d);
        let cubie_w = self.corner_permutation.index(w);
        let cubie_x = self.corner_permutation.index(x);
        let cuvie_y = self.corner_permutation.index(y);
        let cubie_z = self.corner_permutation.index(z);
        match m {
            Moves::L | Moves::R => {
                self.edge_permutation.swap_four_cubicles(a, b, c, d);
                self.corner_permutation.swap_four_cubicles(w, x, y, z);

                self.corner_orientation.add_one(cubie_w);
                self.corner_orientation.add_two(cubie_x);
                self.corner_orientation.add_one(cuvie_y);
                self.corner_orientation.add_two(cubie_z);

                self.edge_orientation.add_one(cubie_a);
                self.edge_orientation.add_one(cubie_b);
                self.edge_orientation.add_one(cubie_c);
                self.edge_orientation.add_one(cubie_d);
            }
            Moves::F | Moves::B => {
                self.edge_permutation.swap_four_cubicles(a, b, c, d);
                self.corner_permutation.swap_four_cubicles(w, x, y, z);

                self.corner_orientation.add_one(cubie_w);
                self.corner_orientation.add_two(cubie_x);
                self.corner_orientation.add_one(cuvie_y);
                self.corner_orientation.add_two(cubie_z);
            }
            Moves::U | Moves::D => {
                self.edge_permutation.swap_four_cubicles(a, b, c, d);
                self.corner_permutation.swap_four_cubicles(w, x, y, z);
            }
            Moves::L_ | Moves::R_ => {
                self.edge_permutation.swap_four_cubicles(d, c, b, a);
                self.corner_permutation.swap_four_cubicles(z, y, x, w);

                self.corner_orientation.add_one(cubie_w);
                self.corner_orientation.add_two(cubie_x);
                self.corner_orientation.add_one(cuvie_y);
                self.corner_orientation.add_two(cubie_z);

                self.edge_orientation.add_one(cubie_a);
                self.edge_orientation.add_one(cubie_b);
                self.edge_orientation.add_one(cubie_c);
                self.edge_orientation.add_one(cubie_d);
            }
            Moves::F_ | Moves::B_ => {
                self.edge_permutation.swap_four_cubicles(d, c, b, a);
                self.corner_permutation.swap_four_cubicles(z, y, x, w);

                self.corner_orientation.add_one(cubie_w);
                self.corner_orientation.add_two(cubie_x);
                self.corner_orientation.add_one(cuvie_y);
                self.corner_orientation.add_two(cubie_z);
            }
            Moves::U_ | Moves::D_ => {
                self.edge_permutation.swap_four_cubicles(d, c, b, a);
                self.corner_permutation.swap_four_cubicles(z, y, x, w);
            }
            Moves::L2 | Moves::R2 | Moves::F2 | Moves::B2 | Moves::U2 | Moves::D2 => {
                self.edge_permutation.swap_two_cubicles(a, c);
                self.edge_permutation.swap_two_cubicles(b, d);
                self.corner_permutation.swap_two_cubicles(w, y);
                self.corner_permutation.swap_two_cubicles(x, z);
            }
        }
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let corner_cubies: Vec<Corner> = (0..NUM_CORNERS)
            .into_iter()
            .map(|idx| {
                let cubie_idx = self.corner_permutation.index(idx);
                (cubie_idx, CORNER_CUBIES[cubie_idx as usize].clone())
            })
            .map(|(idx, corner)| -> Corner {
                corner.orient_corner(self.corner_orientation.orientation_at_index(idx))
            })
            .collect();

        let edge_cubies: Vec<Edge> = (0..NUM_EDGES)
            .into_iter()
            .map(|idx| {
                let cubie_idx = self.edge_permutation.index(idx);
                (cubie_idx, EDGE_CUBIES[cubie_idx as usize].clone())
            })
            .map(|(idx, edge)| -> Edge {
                edge.orient_edge(self.edge_orientation.orientation_at_index(idx))
            })
            .collect();

        let top_face = format!(
            "
                   {} {} {}
                   {} Y {}
                   {} {} {}
                   ",
            corner_cubies[0].facelet_a(),
            edge_cubies[0].facelet_a(),
            corner_cubies[1].facelet_a(),
            edge_cubies[3].facelet_a(),
            edge_cubies[1].facelet_a(),
            corner_cubies[3].facelet_a(),
            edge_cubies[2].facelet_a(),
            corner_cubies[2].facelet_a()
        );

        let middle_faces = format!(
            "
            {} {} {}  {} {} {}  {} {} {}  {} {} {}
            {} R {}  {} G {}  {} O {}  {} B {}
            {} {} {}  {} {} {}  {} {} {}  {} {} {}
                                 ",
            corner_cubies[0].facelet_b(),
            edge_cubies[3].facelet_b(),
            corner_cubies[3].facelet_c(),
            corner_cubies[3].facelet_b(),
            edge_cubies[2].facelet_b(),
            corner_cubies[2].facelet_c(),
            corner_cubies[2].facelet_b(),
            edge_cubies[1].facelet_b(),
            corner_cubies[1].facelet_c(),
            corner_cubies[1].facelet_b(),
            edge_cubies[0].facelet_b(),
            corner_cubies[0].facelet_c(),
            edge_cubies[4].facelet_a(),
            edge_cubies[7].facelet_a(),
            edge_cubies[7].facelet_b(),
            edge_cubies[6].facelet_b(),
            edge_cubies[6].facelet_a(),
            edge_cubies[5].facelet_a(),
            edge_cubies[5].facelet_b(),
            edge_cubies[4].facelet_b(),
            corner_cubies[4].facelet_c(),
            edge_cubies[11].facelet_b(),
            corner_cubies[7].facelet_b(),
            corner_cubies[7].facelet_c(),
            edge_cubies[10].facelet_b(),
            corner_cubies[6].facelet_b(),
            corner_cubies[6].facelet_c(),
            edge_cubies[9].facelet_b(),
            corner_cubies[5].facelet_b(),
            corner_cubies[5].facelet_c(),
            edge_cubies[8].facelet_b(),
            corner_cubies[4].facelet_b(),
        );

        let bottom_face = format!(
            "
                   {} {} {}
                   {} W {}
                   {} {} {}
                   ",
            corner_cubies[7].facelet_a(),
            edge_cubies[10].facelet_a(),
            corner_cubies[6].facelet_a(),
            edge_cubies[11].facelet_a(),
            edge_cubies[9].facelet_a(),
            corner_cubies[4].facelet_a(),
            edge_cubies[8].facelet_a(),
            corner_cubies[5].facelet_a()
        );

        write!(f, "{}{}{}", top_face, middle_faces, bottom_face)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cube;

    #[test]
    fn cube_sanity_test() {
        let cube = Cube::new();
        assert_eq!(
            cube.edge_permutation.parity(),
            cube.corner_permutation.parity()
        );

        assert_eq!(cube.edge_orientation.sum() % 2, 0);
        assert_eq!(cube.corner_orientation.sum() % 3, 0);
    }
    #[test]
    fn cube_undo_turn_test() {
        let solved_cube = Cube::new();
        let mut cube = Cube::new();

        cube.turn(crate::Moves::L);
        cube.turn(crate::Moves::L_);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::L2);
        cube.turn(crate::Moves::L2);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::R);
        cube.turn(crate::Moves::R_);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::R2);
        cube.turn(crate::Moves::R2);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::U);
        cube.turn(crate::Moves::U_);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::U2);
        cube.turn(crate::Moves::U2);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::D);
        cube.turn(crate::Moves::D_);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::D2);
        cube.turn(crate::Moves::D2);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::F);
        cube.turn(crate::Moves::F_);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::F2);
        cube.turn(crate::Moves::F2);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::B);
        cube.turn(crate::Moves::B_);

        assert_eq!(cube, solved_cube);

        cube.turn(crate::Moves::B2);
        cube.turn(crate::Moves::B2);

        assert_eq!(cube, solved_cube);
    }

    #[test]
    fn cube_turns_test() {
        let _cube = Cube::scramble(32);
    }

    #[test]
    fn smove_test() {
        let mut cube = Cube::new();
        let solved_cube = Cube::new();

        for _ in 0..7 {
            cube.turn(crate::Moves::U);
            cube.turn(crate::Moves::R);
            cube.turn(crate::Moves::R_);
            cube.turn(crate::Moves::U_);
        }

        assert_eq!(cube, solved_cube);
    }

    #[test]
    fn cube_solvable_test() {
        let cube = Cube::scramble(32);

        assert_eq!(
            cube.edge_permutation.parity(),
            cube.corner_permutation.parity()
        );

        assert_eq!(cube.edge_orientation.sum() % 2, 0);
        assert_eq!(cube.corner_orientation.sum() % 3, 0);
    }

    #[test]
    fn random_scramble() {
        // B' R U2 R U R' L' U2 F B' D2 F2 D' L B2 D2 R2 L D' L D2 R L2 B' R'
        let mut cube = Cube::new();

        cube.turn(crate::Moves::B_);
        cube.turn(crate::Moves::R);
        cube.turn(crate::Moves::U2);
        cube.turn(crate::Moves::R);
        cube.turn(crate::Moves::U);
        cube.turn(crate::Moves::R_);
        cube.turn(crate::Moves::L_);
        cube.turn(crate::Moves::U2);
        cube.turn(crate::Moves::F);
        cube.turn(crate::Moves::B_);
        cube.turn(crate::Moves::D2);
        cube.turn(crate::Moves::F2);
        cube.turn(crate::Moves::D_);
        cube.turn(crate::Moves::L);
        cube.turn(crate::Moves::B2);
        cube.turn(crate::Moves::D2);
        cube.turn(crate::Moves::R2);
        cube.turn(crate::Moves::L);
        cube.turn(crate::Moves::D_);
        cube.turn(crate::Moves::L);
        cube.turn(crate::Moves::D2);
        cube.turn(crate::Moves::R);
        cube.turn(crate::Moves::L2);
        cube.turn(crate::Moves::B_);
        cube.turn(crate::Moves::R_);

        println!("{}", cube);
        assert!(false);
    }
}
