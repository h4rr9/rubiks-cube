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
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
        facelet_c: Faces::Blue,
    },
    Corner {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Orange,
    },
    Corner {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Green,
    },
    Corner {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
        facelet_c: Faces::Red,
    },
    Corner {
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
        facelet_c: Faces::Red,
    },
    Corner {
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
        facelet_c: Faces::Blue,
    },
    Corner {
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
        facelet_c: Faces::Orange,
    },
    Corner {
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
        facelet_c: Faces::Green,
    },
];

pub const EDGE_CUBIES: [Edge; NUM_EDGES as usize] = [
    Edge {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Blue,
    },
    Edge {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Orange,
    },
    Edge {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Green,
    },
    Edge {
        facelet_a: Faces::Yellow,
        facelet_b: Faces::Red,
    },
    Edge {
        facelet_a: Faces::Red,
        facelet_b: Faces::Blue,
    },
    Edge {
        facelet_a: Faces::Orange,
        facelet_b: Faces::Blue,
    },
    Edge {
        facelet_a: Faces::Orange,
        facelet_b: Faces::Green,
    },
    Edge {
        facelet_a: Faces::Red,
        facelet_b: Faces::Green,
    },
    Edge {
        facelet_a: Faces::White,
        facelet_b: Faces::Blue,
    },
    Edge {
        facelet_a: Faces::White,
        facelet_b: Faces::Orange,
    },
    Edge {
        facelet_a: Faces::White,
        facelet_b: Faces::Green,
    },
    Edge {
        facelet_a: Faces::White,
        facelet_b: Faces::Red,
    },
];
