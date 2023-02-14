use light_arrangements::Color;
use pyo3::{exceptions::PyValueError, PyResult};

/// Converts a vector to an array
pub fn vec_to_array<const N: usize>(vec: Vec<f64>) -> PyResult<[f64; N]> {
    if vec.len() == N {
        let vec_len = vec.len();
        return match vec.try_into() {
            Ok(array) => Ok(array),
            Err(_) => Err(PyValueError::new_err(format!(
                "expected a vec of length {} but it was {}",
                N, vec_len,
            ))),
        };
    } else {
        return Err(PyValueError::new_err(format!(
            "Expected input of dimension {} but was {}",
            N,
            vec.len()
        )));
    }
}

/// Converts the first 3 elements of a vector to a Color
pub fn vec_to_color(vec: &Vec<u8>) -> Color {
    Color {
        red: vec[0],
        green: vec[1],
        blue: vec[2],
    }
}
