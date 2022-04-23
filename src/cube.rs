//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com

use std::fmt::Display;

use std::str::FromStr;

use crate::{
    cubies::*,
    errors::CubeError,
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

    /// Initializes a Cube object with values from 6 x 3 x 3 array of face colors.
    /// The function assumes Green face to the front and Yellow facing up.
    /// The expected order of face colours is W, Y, G, B, R, O
    ///
    /// # Arguments
    ///
    /// * `cube_arr` - 6 x 3 x 3 array of face colors
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::rubikscube::Cube;
    /// let cube_array =
    /// [[["W";3];3],[["Y";3];3],[["G";3];3],[["B";3];3],[["R";3];3],[["O";3];3]];
    /// let cube = Cube::cube_from_array(&cube_array);
    /// ```
    pub fn cube_from_array(cube_array: &[[[&str; 3]; 3]; 6]) -> Result<Cube, CubeError> {
        let mut cube_faces = [[[Faces::White; 3]; 3]; 6];

        for (i, face) in cube_array.iter().enumerate() {
            for (j, row) in face.into_iter().enumerate() {
                for (k, elem) in row.iter().enumerate() {
                    let face: Faces = Faces::from_str(elem)?;
                    if j == 1 && k == 1 && face as usize != i {
                        return Err(CubeError::InvalidFaceOrder(face, i as usize));
                    }
                    cube_faces[i][j][k] = face;
                }
            }
        }

        Ok(Cube::cube_from_faces(&cube_faces))
    }

    fn cube_from_faces(cube_faces: &[[[Faces; 3]; 3]; 6]) -> Cube {
        let mut edge_permutation = vec![0u8; NUM_EDGES as usize];
        let mut corner_permutation = vec![0u8; NUM_CORNERS as usize];
        let mut edge_orientation = EdgeOrientation::new();
        let mut corner_orientation = CornerOrientation::new();

        // sets corner cubies orientation and values
        for (corner_idx, corner) in CORNER_CUBIES.iter().enumerate() {
            let primary_facelet_idx = corner.cubie_facelet_a_idx().unwrap();
            let secondary_facelet_idx = corner.cubie_facelet_b_idx().unwrap();
            let tertiary_facelet_idx = corner.cubie_facelet_c_idx().unwrap();

            let facelet_a = cube_faces[primary_facelet_idx.0 as usize]
                [primary_facelet_idx.1 as usize][primary_facelet_idx.2 as usize];
            let facelet_b = cube_faces[secondary_facelet_idx.0 as usize]
                [secondary_facelet_idx.1 as usize][secondary_facelet_idx.2 as usize];
            let facelet_c = cube_faces[tertiary_facelet_idx.0 as usize]
                [tertiary_facelet_idx.1 as usize][tertiary_facelet_idx.2 as usize];

            let corner_cubie = Corner::new(facelet_a, facelet_b, facelet_c);

            let corner_cubie_idx = corner_cubie.cubie_index();
            corner_permutation[corner_idx] = corner_cubie_idx;

            let primary_facelet: Faces = CORNER_CUBIES[corner_cubie_idx as usize].facelet_a();

            match corner_cubie.get_orientation(primary_facelet) {
                1 => corner_orientation.add_two(corner_cubie_idx),
                2 => corner_orientation.add_one(corner_cubie_idx),
                _ => {}
            }
        }

        // sets edge cubies orientation and values
        for (edge_idx, edge) in EDGE_CUBIES.iter().enumerate() {
            let primary_facelet_idx = edge.cubie_facelet_a_idx().unwrap();
            let secondary_facelet_idx = edge.cubie_facelet_b_idx().unwrap();

            let facelet_a = cube_faces[primary_facelet_idx.0 as usize]
                [primary_facelet_idx.1 as usize][primary_facelet_idx.2 as usize];
            let facelet_b = cube_faces[secondary_facelet_idx.0 as usize]
                [secondary_facelet_idx.1 as usize][secondary_facelet_idx.2 as usize];

            let edge_cubie = Edge::new(facelet_a, facelet_b);

            let edge_cubie_idx = edge_cubie.cubie_index();
            edge_permutation[edge_idx] = edge_cubie_idx;

            let primary_facelet: Faces = EDGE_CUBIES[edge_cubie_idx as usize].facelet_a();

            match edge_cubie.get_orientation(primary_facelet) {
                1 => edge_orientation.add_one(edge_cubie_idx),
                _ => {}
            }
        }

        let edge_permutation = Permutation::new_with_permutation(&edge_permutation);
        let corner_permutation = Permutation::new_with_permutation(&corner_permutation);

        Cube {
            edge_orientation,
            corner_orientation,
            edge_permutation,
            corner_permutation,
        }
    }

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
        // unpack cubicle indices
        let ((a, b, c, d), (w, x, y, z)) = match m {
            Moves::L | Moves::L_ | Moves::L2 => (L_EDGE_CUBICLES, L_CORNER_CUBICLES),
            Moves::R | Moves::R_ | Moves::R2 => (R_EDGE_CUBICLES, R_CORNER_CUBICLES),
            Moves::F | Moves::F_ | Moves::F2 => (F_EDGE_CUBICLES, F_CORNER_CUBICLES),
            Moves::B | Moves::B_ | Moves::B2 => (B_EDGE_CUBICLES, B_CORNER_CUBICLES),
            Moves::U | Moves::U_ | Moves::U2 => (U_EDGE_CUBICLES, U_CORNER_CUBICLES),
            Moves::D | Moves::D_ | Moves::D2 => (D_EDGE_CUBICLES, D_CORNER_CUBICLES),
        };

        // updating edge and corner cubie orientation based on move
        match m {
            Moves::L | Moves::R | Moves::L_ | Moves::R_ => {
                let cubie_a = self.edge_permutation.cubie_in_cubicle(a);
                let cubie_b = self.edge_permutation.cubie_in_cubicle(b);
                let cubie_c = self.edge_permutation.cubie_in_cubicle(c);
                let cubie_d = self.edge_permutation.cubie_in_cubicle(d);
                let cubie_w = self.corner_permutation.cubie_in_cubicle(w);
                let cubie_x = self.corner_permutation.cubie_in_cubicle(x);
                let cubie_y = self.corner_permutation.cubie_in_cubicle(y);
                let cubie_z = self.corner_permutation.cubie_in_cubicle(z);

                self.corner_orientation.add_one(cubie_w);
                self.corner_orientation.add_two(cubie_x);
                self.corner_orientation.add_one(cubie_y);
                self.corner_orientation.add_two(cubie_z);

                self.edge_orientation.add_one(cubie_a);
                self.edge_orientation.add_one(cubie_b);
                self.edge_orientation.add_one(cubie_c);
                self.edge_orientation.add_one(cubie_d);
            }
            Moves::F | Moves::B | Moves::F_ | Moves::B_ => {
                let cubie_w = self.corner_permutation.cubie_in_cubicle(w);
                let cubie_x = self.corner_permutation.cubie_in_cubicle(x);
                let cubie_y = self.corner_permutation.cubie_in_cubicle(y);
                let cubie_z = self.corner_permutation.cubie_in_cubicle(z);

                self.corner_orientation.add_one(cubie_w);
                self.corner_orientation.add_two(cubie_x);
                self.corner_orientation.add_one(cubie_y);
                self.corner_orientation.add_two(cubie_z);
            }
            _ => {}
        }

        // updating edge and corner cubie permutation based on move
        match m {
            Moves::L | Moves::R | Moves::F | Moves::B | Moves::U | Moves::D => {
                self.edge_permutation.swap_four_cubicles(a, b, c, d);
                self.corner_permutation.swap_four_cubicles(w, x, y, z);
            }
            Moves::L_ | Moves::R_ | Moves::F_ | Moves::B_ | Moves::U_ | Moves::D_ => {
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

    pub fn is_solvable(&self) -> bool {
        (self.edge_permutation.parity() == self.corner_permutation.parity())
            && (self.edge_orientation.sum() % 2 == 0)
            && (self.corner_orientation.sum() % 3 == 0)
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let corner_cubies: Vec<Corner> = (0..NUM_CORNERS)
            .into_iter()
            .map(|idx| {
                let cubie_idx = self.corner_permutation.cubie_in_cubicle(idx);
                (cubie_idx, CORNER_CUBIES[cubie_idx as usize].clone())
            })
            .map(|(idx, corner)| -> Corner {
                corner.orient_corner(self.corner_orientation.orientation_at_index(idx))
            })
            .collect();

        let edge_cubies: Vec<Edge> = (0..NUM_EDGES)
            .into_iter()
            .map(|idx| {
                let cubie_idx = self.edge_permutation.cubie_in_cubicle(idx);
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

    use crate::cubies::Faces;
    use crate::errors::CubeError;
    use crate::Cube;
    use crate::Moves::L;

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

        cube.turn(L);
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
        cube.turn(L);
        cube.turn(crate::Moves::B2);
        cube.turn(crate::Moves::D2);
        cube.turn(crate::Moves::R2);
        cube.turn(L);
        cube.turn(crate::Moves::D_);
        cube.turn(L);
        cube.turn(crate::Moves::D2);
        cube.turn(crate::Moves::R);
        cube.turn(crate::Moves::L2);
        cube.turn(crate::Moves::B_);
        cube.turn(crate::Moves::R_);

        println!("{}", cube);

        let cube_array = [
            [["O", "Y", "O"], ["G", "W", "B"], ["O", "G", "B"]], // W
            [["B", "W", "R"], ["B", "Y", "Y"], ["R", "O", "G"]], // Y
            [["G", "Y", "Y"], ["B", "G", "R"], ["Y", "R", "B"]], // G
            [["B", "O", "W"], ["G", "B", "R"], ["Y", "O", "W"]], // B
            [["R", "W", "W"], ["G", "R", "R"], ["G", "Y", "G"]], // R
            [["R", "B", "Y"], ["W", "O", "W"], ["W", "O", "O"]], // O
        ];

        let cube_from_array = Cube::cube_from_array(&cube_array).unwrap();

        assert!(cube_from_array.is_solvable());

        println!("{}", cube_from_array);
        assert_eq!(cube_from_array, cube);
    }

    #[test]
    fn from_array_sanity_test() {
        let cube_array = [
            [["W"; 3]; 3],
            [["Y"; 3]; 3],
            [["G"; 3]; 3],
            [["B"; 3]; 3],
            [["R"; 3]; 3],
            [["O"; 3]; 3],
        ];
        let cube = Cube::cube_from_array(&cube_array).unwrap();

        let solved_cube = Cube::new();

        assert_eq!(cube, solved_cube);
    }

    #[test]
    fn from_array_order_err_test() {
        let cube_array = [
            [["W"; 3]; 3],
            [["Y"; 3]; 3],
            [["B"; 3]; 3],
            [["G"; 3]; 3],
            [["R"; 3]; 3],
            [["O"; 3]; 3],
        ];
        let cube_err = Cube::cube_from_array(&cube_array).unwrap_err();

        assert_eq!(cube_err, CubeError::InvalidFaceOrder(Faces::Blue, 2));
    }

    #[test]
    fn from_array_color_err_test() {
        let cube_array = [
            [["P"; 3]; 3],
            [["Y"; 3]; 3],
            [["G"; 3]; 3],
            [["B"; 3]; 3],
            [["R"; 3]; 3],
            [["O"; 3]; 3],
        ];
        let cube_err = Cube::cube_from_array(&cube_array).unwrap_err();

        assert_eq!(cube_err, CubeError::InvalidColor);
    }
}
