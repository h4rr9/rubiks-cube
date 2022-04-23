use std::fmt;

use crate::cubies::Faces;

#[derive(Debug, PartialEq)]
pub enum CubeError {
    InvalidFaceOrder(Faces, usize),
    InvalidColor,
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
            CubeError::InvalidColor => write!(f, "Invalid color"),
        }
    }
}

impl From<strum::ParseError> for CubeError {
    fn from(_: strum::ParseError) -> Self {
        CubeError::InvalidColor
    }
}
