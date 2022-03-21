use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Faces {
    White,
    Yellow,
    Green,
    Blue,
    Red,
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
    cubie_facelet_a_idx: (Faces, u8, u8),
    cubie_facelet_b_idx: (Faces, u8, u8),
    cubie_facelet_c_idx: (Faces, u8, u8),
}
impl Corner {
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
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Edge {
    facelet_a: Faces,
    facelet_b: Faces,
    cubie_facelet_a_idx: (Faces, u8, u8),
    cubie_facelet_b_idx: (Faces, u8, u8),
}

impl Edge {
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

    /// Get the edge's facelet a.
    pub fn facelet_a(&self) -> Faces {
        self.facelet_a
    }

    /// Get the edge's facelet b.
    pub fn facelet_b(&self) -> Faces {
        self.facelet_b
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
        cubie_facelet_a_idx: (Faces::Yellow, 0, 0),
        cubie_facelet_b_idx: (Faces::Red, 0, 0),
        cubie_facelet_c_idx: (Faces::Blue, 0, 2),
    },
    Corner {
        // cubie 1
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Orange,
        cubie_facelet_a_idx: (Faces::Yellow, 0, 2),
        cubie_facelet_b_idx: (Faces::Blue, 0, 0),
        cubie_facelet_c_idx: (Faces::Orange, 0, 2),
    },
    Corner {
        // cubie 2
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Green,
        cubie_facelet_a_idx: (Faces::Yellow, 2, 2),
        cubie_facelet_b_idx: (Faces::Orange, 0, 0),
        cubie_facelet_c_idx: (Faces::Green, 0, 2),
    },
    Corner {
        // cubie 3
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
        facelet_c: Faces::Red,
        cubie_facelet_a_idx: (Faces::Yellow, 2, 0),
        cubie_facelet_b_idx: (Faces::Green, 0, 0),
        cubie_facelet_c_idx: (Faces::Red, 0, 2),
    },
    Corner {
        // cubie 4
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Red,
        cubie_facelet_a_idx: (Faces::White, 2, 0),
        cubie_facelet_b_idx: (Faces::Blue, 2, 2),
        cubie_facelet_c_idx: (Faces::Red, 2, 0),
    },
    Corner {
        // cubie 5
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Blue,
        cubie_facelet_a_idx: (Faces::White, 2, 2),
        cubie_facelet_b_idx: (Faces::Orange, 2, 2),
        cubie_facelet_c_idx: (Faces::Blue, 2, 0),
    },
    Corner {
        // cubie 6
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
        facelet_c: Faces::Orange,
        cubie_facelet_a_idx: (Faces::White, 0, 2),
        cubie_facelet_b_idx: (Faces::Green, 2, 2),
        cubie_facelet_c_idx: (Faces::Orange, 2, 0),
    },
    Corner {
        // cubie 7
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
        facelet_c: Faces::Green,
        cubie_facelet_a_idx: (Faces::White, 0, 0),
        cubie_facelet_b_idx: (Faces::Red, 2, 2),
        cubie_facelet_c_idx: (Faces::Green, 2, 0),
    },
];

pub const EDGE_CUBIES: [Edge; NUM_EDGES as usize] = [
    Edge {
        // cubie 0
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: (Faces::Yellow, 0, 1),
        cubie_facelet_b_idx: (Faces::Blue, 0, 1),
    },
    Edge {
        // cubie 1
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
        cubie_facelet_a_idx: (Faces::Yellow, 1, 2),
        cubie_facelet_b_idx: (Faces::Orange, 0, 1),
    },
    Edge {
        // cubie 2
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: (Faces::Yellow, 2, 1),
        cubie_facelet_b_idx: (Faces::Green, 0, 1),
    },
    Edge {
        // cubie 3
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
        cubie_facelet_a_idx: (Faces::Yellow, 1, 0),
        cubie_facelet_b_idx: (Faces::Red, 0, 1),
    },
    Edge {
        // cubie 4
        facelet_a: Faces::Red,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: (Faces::Red, 1, 0),
        cubie_facelet_b_idx: (Faces::Blue, 2, 1),
    },
    Edge {
        // cubie 5
        facelet_a: Faces::Orange,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: (Faces::Orange, 1, 2),
        cubie_facelet_b_idx: (Faces::Blue, 0, 1),
    },
    Edge {
        // cubie 6
        facelet_a: Faces::Orange,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: (Faces::Orange, 1, 0),
        cubie_facelet_b_idx: (Faces::Green, 1, 2),
    },
    Edge {
        // cubie 7
        facelet_a: Faces::Red,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: (Faces::Red, 1, 2),
        cubie_facelet_b_idx: (Faces::Green, 1, 0),
    },
    Edge {
        // cubie 8
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
        cubie_facelet_a_idx: (Faces::White, 2, 1),
        cubie_facelet_b_idx: (Faces::Blue, 2, 1),
    },
    Edge {
        // cubie 9
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
        cubie_facelet_a_idx: (Faces::White, 1, 2),
        cubie_facelet_b_idx: (Faces::Orange, 2, 1),
    },
    Edge {
        // cubie 10
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
        cubie_facelet_a_idx: (Faces::White, 0, 1),
        cubie_facelet_b_idx: (Faces::Green, 2, 1),
    },
    Edge {
        // cubie 11
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
        cubie_facelet_a_idx: (Faces::White, 1, 0),
        cubie_facelet_b_idx: (Faces::Red, 2, 1),
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
