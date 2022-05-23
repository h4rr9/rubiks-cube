use crate::cube::Cube;
use crate::moves::{MetricKind, Turn};

#[cfg(feature = "python")]
use pyo3::{exceptions::PyIndexError, prelude::*, types::PyType};

#[cfg(feature = "python")]
use numpy::array::PyArray1;

#[cfg(feature = "python")]
#[pyclass(name = "Cube")]
struct PyCube(Cube, Vec<PyTurn>);

#[cfg(feature = "python")]
#[pymethods]
impl PyCube {
    #[new]
    fn new(turn_metric: &PyMetric) -> Self {
        PyCube(
            Cube::new(turn_metric.0),
            Self::all_possible_turns().unwrap(),
        )
    }

    #[classmethod]
    fn cube_htm(_py: &PyType) -> Self {
        PyCube(Cube::cube_htm(), Self::all_possible_turns().unwrap())
    }

    #[classmethod]
    fn cube_qtm(_py: &PyType) -> Self {
        PyCube(Cube::cube_qtm(), Self::all_possible_turns().unwrap())
    }

    #[args(num_turns = "100")]
    fn scramble(&mut self, num_turns: u32) {
        self.0.scramble(num_turns);
    }

    fn turn(&mut self, twist: u8) -> PyResult<()> {
        if let Err(err) = self.0.turn(twist) {
            Err(PyIndexError::new_err(err.to_string()))
        } else {
            Ok(())
        }
    }

    fn solved(&self) -> bool {
        self.0.solved()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }

    #[staticmethod]
    fn all_possible_turns() -> PyResult<Vec<PyTurn>> {
        Ok(vec![
            PyTurn(Turn::L),  // 0
            PyTurn(Turn::R),  // 1
            PyTurn(Turn::F),  // 2
            PyTurn(Turn::B),  // 3
            PyTurn(Turn::U),  // 4
            PyTurn(Turn::D),  // 5
            PyTurn(Turn::L_), // 6
            PyTurn(Turn::R_), // 7
            PyTurn(Turn::F_), // 8
            PyTurn(Turn::B_), // 9
            PyTurn(Turn::U_), // 10
            PyTurn(Turn::D_), // 11
            PyTurn(Turn::L2), // 12
            PyTurn(Turn::R2), // 13
            PyTurn(Turn::F2), // 14
            PyTurn(Turn::B2), // 15
            PyTurn(Turn::U2), // 16
            PyTurn(Turn::D2), // 17
        ])
    }

    fn representation<'a>(&self, _py: Python<'a>) -> &'a PyArray1<bool> {
        PyArray1::from_slice(_py, &self.0.representation())
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "Metric")]
struct PyMetric(MetricKind);

#[cfg(feature = "python")]
#[pymethods]
impl PyMetric {
    fn to_int(&self) -> u8 {
        self.0 as u8
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "Turn")]
struct PyTurn(Turn);

#[cfg(feature = "python")]
#[pymethods]
impl PyTurn {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
fn rubikscube(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCube>()?;
    m.add_class::<PyTurn>()?;
    m.add_class::<PyMetric>()?;
    m.add("HalfTurnMetric", PyMetric(MetricKind::HalfTurnMetric))?;
    m.add("QuarterTurnMetric", PyMetric(MetricKind::QuarterTurnMetric))?;
    Ok(())
}
