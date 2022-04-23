use std::fmt;

use crate::cubies::Faces;

#[derive(Debug, PartialEq)]
pub enum CubeError {
    InvalidFaceOrder(Faces, usize),
    InvalidFaceletColor,
    InvalidTurn,
}

impl std::error::Error for CubeError {}

impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CubeError::InvalidFaceOrder(face_found, index) => write!(
                f,
                "Invalid order found, Expected {} at face index {}, found {}",
                index,
                Faces::from_repr(*index).unwrap(),
                face_found
            ),
            CubeError::InvalidFaceletColor => write!(f, "Invalid Facelet color"),
            CubeError::InvalidTurn => write!(f, "Invalid Turn"),
        }
    }
}

impl From<strum::ParseError> for CubeError {
    fn from(_: strum::ParseError) -> Self {
        CubeError::InvalidFaceletColor
    }
}
