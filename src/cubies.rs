#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum Faces {
    White,
    Yellow,
    Green,
    Blue,
    Red,
    Orange,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Corner {
    facelet_a: Faces,
    facelet_b: Faces,
    facelet_c: Faces,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Edge {
    facelet_a: Faces,
    facelet_b: Faces,
}

pub const NUM_CORNERS: u8 = 8;
pub const NUM_EDGES: u8 = 12;

pub const CORNER_CUBIES: [Corner; NUM_CORNERS as usize] = [
    Corner {
        // cubie 0
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
        facelet_c: Faces::Blue,
    },
    Corner {
        // cubie 1
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Orange,
    },
    Corner {
        // cubie 2
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Green,
    },
    Corner {
        // cubie 3
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
        facelet_c: Faces::Red,
    },
    Corner {
        // cubie 4
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Red,
    },
    Corner {
        // cubie 5
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Blue,
    },
    Corner {
        // cubie 6
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
        facelet_c: Faces::Orange,
    },
    Corner {
        // cubie 7
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
        facelet_c: Faces::Green,
    },
];

pub const EDGE_CUBIES: [Edge; NUM_EDGES as usize] = [
    Edge { // cubie 0
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
    },
    Edge {// cubie 1
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
    },
    Edge {// cubie 2
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
    },
    Edge {// cubie 3
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
    },
    Edge {// cubie 4
        facelet_a: Faces::Red,
        facelet_b: Faces::Blue,
    },
    Edge {// cubie 5
        facelet_a: Faces::Orange,
        facelet_b: Faces::Blue,
    },
    Edge {// cubie 6
        facelet_a: Faces::Orange,
        facelet_b: Faces::Green,
    },
    Edge {// cubie 7
        facelet_a: Faces::Red,
        facelet_b: Faces::Green,
    },
    Edge {// cubie 8
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
    },
    Edge {// cubie 9
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
    },
    Edge {// cubie 10
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
    },
    Edge {// cubie 11
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
    },
];
