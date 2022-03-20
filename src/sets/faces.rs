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
    first_face: Faces,
    second_face: Faces,
    third_face: Faces,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Edge {
    first_face: Faces,
    second_face: Faces,
}

pub const NUM_CORNERS: u8 = 8;
pub const NUM_EDGES: u8 = 12;

pub const CORNERS: [Corner; NUM_CORNERS as usize] = [
    Corner {
        first_face: Faces::Yellow,
        second_face: Faces::Red,
        third_face: Faces::Blue,
    },
    Corner {
        first_face: Faces::Yellow,
        second_face: Faces::Blue,
        third_face: Faces::Orange,
    },
    Corner {
        first_face: Faces::Yellow,
        second_face: Faces::Orange,
        third_face: Faces::Green,
    },
    Corner {
        first_face: Faces::Yellow,
        second_face: Faces::Green,
        third_face: Faces::Red,
    },
    Corner {
        first_face: Faces::White,
        second_face: Faces::Blue,
        third_face: Faces::Red,
    },
    Corner {
        first_face: Faces::White,
        second_face: Faces::Orange,
        third_face: Faces::Blue,
    },
    Corner {
        first_face: Faces::White,
        second_face: Faces::Green,
        third_face: Faces::Orange,
    },
    Corner {
        first_face: Faces::White,
        second_face: Faces::Red,
        third_face: Faces::Green,
    },
];

pub const EDGES: [Edge; NUM_EDGES as usize] = [
    Edge {
        first_face: Faces::Yellow,
        second_face: Faces::Blue,
    },
    Edge {
        first_face: Faces::Yellow,
        second_face: Faces::Orange,
    },
    Edge {
        first_face: Faces::Yellow,
        second_face: Faces::Green,
    },
    Edge {
        first_face: Faces::Yellow,
        second_face: Faces::Red,
    },
    Edge {
        first_face: Faces::Red,
        second_face: Faces::Blue,
    },
    Edge {
        first_face: Faces::Orange,
        second_face: Faces::Blue,
    },
    Edge {
        first_face: Faces::Orange,
        second_face: Faces::Green,
    },
    Edge {
        first_face: Faces::Red,
        second_face: Faces::Green,
    },
    Edge {
        first_face: Faces::White,
        second_face: Faces::Blue,
    },
    Edge {
        first_face: Faces::White,
        second_face: Faces::Orange,
    },
    Edge {
        first_face: Faces::White,
        second_face: Faces::Green,
    },
    Edge {
        first_face: Faces::White,
        second_face: Faces::Red,
    },
];
