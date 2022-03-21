use std::cmp::Eq;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Copy, Clone, EnumString)]
pub enum Faces {
    #[strum(serialize = "white", serialize = "W")]
    White,
    #[strum(serialize = "yellow", serialize = "Y")]
    Yellow,
    #[strum(serialize = "green", serialize = "G")]
    Green,
    #[strum(serialize = "blue", serialize = "B")]
    Blue,
    #[strum(serialize = "red", serialize = "R")]
    Red,
    #[strum(serialize = "orange", serialize = "O")]
    Orange,
}

impl Display for Faces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Faces::White => "W",
                Faces::Yellow => "Y",
                Faces::Green => "G",
                Faces::Blue => "B",
                Faces::Red => "R",
                Faces::Orange => "O",
            }
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Corner {
    facelet_a: Faces,
    facelet_b: Faces,
    facelet_c: Faces,
    cubie_facelet_a_idx: Option<(Faces, u8, u8)>,
    cubie_facelet_b_idx: Option<(Faces, u8, u8)>,
    cubie_facelet_c_idx: Option<(Faces, u8, u8)>,
}
impl Corner {
    pub fn new(facelet_a: Faces, facelet_b: Faces, facelet_c: Faces) -> Corner {
        Corner {
            facelet_a,
            facelet_b,
            facelet_c,
            cubie_facelet_a_idx: None,
            cubie_facelet_b_idx: None,
            cubie_facelet_c_idx: None,
        }
    }
    pub fn get_orientation(&self, expected_facelet: Faces) -> u8 {
        if expected_facelet == self.facelet_a {
            0
        } else if expected_facelet == self.facelet_b {
            1
        } else if expected_facelet == self.facelet_c {
            2
        } else {
            panic!("invalid cubie encoutered, {:?} not found", expected_facelet);
        }
    }
    pub fn orient_corner(self, orientation: u8) -> Corner {
        match orientation {
            0 => self,
            1 => Corner {
                facelet_a: self.facelet_b,
                facelet_b: self.facelet_c,
                facelet_c: self.facelet_a,
                ..self
            },
            2 => Corner {
                facelet_a: self.facelet_c,
                facelet_b: self.facelet_a,
                facelet_c: self.facelet_b,
                ..self
            },

            o => panic!("invalid corner orientation encountered {}", o),
        }
    }

    pub fn cubie_index(&self) -> u8 {
        let mut cubie_face_set = HashSet::new();
        cubie_face_set.insert(self.facelet_a);
        cubie_face_set.insert(self.facelet_b);
        cubie_face_set.insert(self.facelet_c);

        for (i, corner) in CORNER_CUBIES.iter().enumerate() {
            let mut corner_face_set = HashSet::<Faces>::new();
            corner_face_set.insert(corner.facelet_a());
            corner_face_set.insert(corner.facelet_b());
            corner_face_set.insert(corner.facelet_c());

            if corner_face_set == cubie_face_set {
                return i as u8;
            }
        }

        panic!("Corner cubie not found!")
    }

    /// Get the corner's facelet a.
    pub fn facelet_a(&self) -> Faces {
        self.facelet_a
    }

    /// Get the corner's facelet b.
    pub fn facelet_b(&self) -> Faces {
        self.facelet_b
    }

    /// Get the corner's facelet c.
    pub fn facelet_c(&self) -> Faces {
        self.facelet_c
    }

    /// Get the corner's cubie facelet a idx.
    pub fn cubie_facelet_a_idx(&self) -> Option<(Faces, u8, u8)> {
        self.cubie_facelet_a_idx
    }

    /// Get the corner's cubie facelet b idx.
    pub fn cubie_facelet_b_idx(&self) -> Option<(Faces, u8, u8)> {
        self.cubie_facelet_b_idx
    }

    /// Get the corner's cubie facelet c idx.
    pub fn cubie_facelet_c_idx(&self) -> Option<(Faces, u8, u8)> {
        self.cubie_facelet_c_idx
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Edge {
    facelet_a: Faces,
    facelet_b: Faces,
    cubie_facelet_a_idx: Option<(Faces, u8, u8)>,
    cubie_facelet_b_idx: Option<(Faces, u8, u8)>,
}

impl Edge {
    pub fn new(facelet_a: Faces, facelet_b: Faces) -> Edge {
        Edge {
            facelet_a,
            facelet_b,
            cubie_facelet_a_idx: None,
            cubie_facelet_b_idx: None,
        }
    }

    pub fn orient_edge(self, orientation: u8) -> Edge {
        match orientation {
            0 => self,
            1 => Edge {
                facelet_a: self.facelet_b,
                facelet_b: self.facelet_a,
                ..self
            },
            o => panic!("invalid edge orientation encountered {}", o),
        }
    }

    pub fn get_orientation(&self, expected_facelet: Faces) -> u8 {
        if expected_facelet == self.facelet_a {
            0
        } else if expected_facelet == self.facelet_b {
            1
        } else {
            panic!("invalid cubie encoutered, {:?} not found", expected_facelet);
        }
    }

    pub fn cubie_index(&self) -> u8 {
        let mut cubie_face_set = HashSet::new();
        cubie_face_set.insert(self.facelet_a);
        cubie_face_set.insert(self.facelet_b);

        for (i, edge) in EDGE_CUBIES.iter().enumerate() {
            let mut edge_face_set = HashSet::<Faces>::new();
            edge_face_set.insert(edge.facelet_a());
            edge_face_set.insert(edge.facelet_b());

            if edge_face_set == cubie_face_set {
                return i as u8;
            }
        }

        panic!("Edge cubie not found!")
    }

    /// Get the edge's facelet a.
    pub fn facelet_a(&self) -> Faces {
        self.facelet_a
    }

    /// Get the edge's facelet b.
    pub fn facelet_b(&self) -> Faces {
        self.facelet_b
    }

    /// Get the edge's cubie facelet a idx.

    /// Get the edge's cubie facelet a idx.
    pub fn cubie_facelet_a_idx(&self) -> Option<(Faces, u8, u8)> {
        self.cubie_facelet_a_idx
    }

    /// Get the edge's cubie facelet b idx.
    pub fn cubie_facelet_b_idx(&self) -> Option<(Faces, u8, u8)> {
        self.cubie_facelet_b_idx
    }
}

pub const NUM_CORNERS: u8 = 8;
pub const NUM_EDGES: u8 = 12;

pub const CORNER_CUBIES: [Corner; NUM_CORNERS as usize] = [
    Corner {
        // cubie 0
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
        facelet_c: Faces::Blue,
        cubie_facelet_a_idx: Some((Faces::Yellow, 0, 0)),
        cubie_facelet_b_idx: Some((Faces::Red, 0, 0)),
        cubie_facelet_c_idx: Some((Faces::Blue, 0, 2)),
    },
    Corner {
        // cubie 1
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Orange,
        cubie_facelet_a_idx: Some((Faces::Yellow, 0, 2)),
        cubie_facelet_b_idx: Some((Faces::Blue, 0, 0)),
        cubie_facelet_c_idx: Some((Faces::Orange, 0, 2)),
    },
    Corner {
        // cubie 2
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Green,
        cubie_facelet_a_idx: Some((Faces::Yellow, 2, 2)),
        cubie_facelet_b_idx: Some((Faces::Orange, 0, 0)),
        cubie_facelet_c_idx: Some((Faces::Green, 0, 2)),
    },
    Corner {
        // cubie 3
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
        facelet_c: Faces::Red,
        cubie_facelet_a_idx: Some((Faces::Yellow, 2, 0)),
        cubie_facelet_b_idx: Some((Faces::Green, 0, 0)),
        cubie_facelet_c_idx: Some((Faces::Red, 0, 2)),
    },
    Corner {
        // cubie 4
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Red,
        cubie_facelet_a_idx: Some((Faces::White, 2, 0)),
        cubie_facelet_b_idx: Some((Faces::Blue, 2, 2)),
        cubie_facelet_c_idx: Some((Faces::Red, 2, 0)),
    },
    Corner {
        // cubie 5
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Blue,
        cubie_facelet_a_idx: Some((Faces::White, 2, 2)),
        cubie_facelet_b_idx: Some((Faces::Orange, 2, 2)),
        cubie_facelet_c_idx: Some((Faces::Blue, 2, 0)),
    },
    Corner {
        // cubie 6
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
        facelet_c: Faces::Orange,
        cubie_facelet_a_idx: Some((Faces::White, 0, 2)),
        cubie_facelet_b_idx: Some((Faces::Green, 2, 2)),
        cubie_facelet_c_idx: Some((Faces::Orange, 2, 0)),
    },
    Corner {
        // cubie 7
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
        facelet_c: Faces::Green,
        cubie_facelet_a_idx: Some((Faces::White, 0, 0)),
        cubie_facelet_b_idx: Some((Faces::Red, 2, 2)),
        cubie_facelet_c_idx: Some((Faces::Green, 2, 0)),
    },
];

pub const EDGE_CUBIES: [Edge; NUM_EDGES as usize] = [
    Edge {
        // cubie 0
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: Some((Faces::Yellow, 0, 1)),
        cubie_facelet_b_idx: Some((Faces::Blue, 0, 1)),
    },
    Edge {
        // cubie 1
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
        cubie_facelet_a_idx: Some((Faces::Yellow, 1, 2)),
        cubie_facelet_b_idx: Some((Faces::Orange, 0, 1)),
    },
    Edge {
        // cubie 2
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: Some((Faces::Yellow, 2, 1)),
        cubie_facelet_b_idx: Some((Faces::Green, 0, 1)),
    },
    Edge {
        // cubie 3
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
        cubie_facelet_a_idx: Some((Faces::Yellow, 1, 0)),
        cubie_facelet_b_idx: Some((Faces::Red, 0, 1)),
    },
    Edge {
        // cubie 4
        facelet_a: Faces::Red,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: Some((Faces::Red, 1, 0)),
        cubie_facelet_b_idx: Some((Faces::Blue, 1, 2)),
    },
    Edge {
        // cubie 5
        facelet_a: Faces::Orange,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: Some((Faces::Orange, 1, 2)),
        cubie_facelet_b_idx: Some((Faces::Blue, 1, 0)),
    },
    Edge {
        // cubie 6
        facelet_a: Faces::Orange,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: Some((Faces::Orange, 1, 0)),
        cubie_facelet_b_idx: Some((Faces::Green, 1, 2)),
    },
    Edge {
        // cubie 7
        facelet_a: Faces::Red,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: Some((Faces::Red, 1, 2)),
        cubie_facelet_b_idx: Some((Faces::Green, 1, 0)),
    },
    Edge {
        // cubie 8
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: Some((Faces::White, 2, 1)),
        cubie_facelet_b_idx: Some((Faces::Blue, 2, 1)),
    },
    Edge {
        // cubie 9
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
        cubie_facelet_a_idx: Some((Faces::White, 1, 2)),
        cubie_facelet_b_idx: Some((Faces::Orange, 2, 1)),
    },
    Edge {
        // cubie 10
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: Some((Faces::White, 0, 1)),
        cubie_facelet_b_idx: Some((Faces::Green, 2, 1)),
    },
    Edge {
        // cubie 11
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
        cubie_facelet_a_idx: Some((Faces::White, 1, 0)),
        cubie_facelet_b_idx: Some((Faces::Red, 2, 1)),
    },
];

pub const R_CORNER_CUBICLES: (u8, u8, u8, u8) = (1, 5, 6, 2);
pub const R_EDGE_CUBICLES: (u8, u8, u8, u8) = (1, 5, 9, 6);

pub const L_CORNER_CUBICLES: (u8, u8, u8, u8) = (3, 7, 4, 0);
pub const L_EDGE_CUBICLES: (u8, u8, u8, u8) = (3, 7, 11, 4);

pub const U_CORNER_CUBICLES: (u8, u8, u8, u8) = (0, 1, 2, 3);
pub const U_EDGE_CUBICLES: (u8, u8, u8, u8) = (0, 1, 2, 3);

pub const D_CORNER_CUBICLES: (u8, u8, u8, u8) = (4, 7, 6, 5);
pub const D_EDGE_CUBICLES: (u8, u8, u8, u8) = (8, 11, 10, 9);

pub const F_CORNER_CUBICLES: (u8, u8, u8, u8) = (2, 6, 7, 3);
pub const F_EDGE_CUBICLES: (u8, u8, u8, u8) = (2, 6, 10, 7);

pub const B_CORNER_CUBICLES: (u8, u8, u8, u8) = (0, 4, 5, 1);
pub const B_EDGE_CUBICLES: (u8, u8, u8, u8) = (0, 4, 8, 5);
