use light_arrangement_python_obj::PyLightArrangement;
use pyo3::prelude::*;
use pyo3::{types::PyModule, Python};

mod errors;
mod light_arrangement_python_obj;
mod light_arrangement_thread;
mod types;
mod util;

use light_arrangement_python_obj::init_test;
use light_arrangement_python_obj::init_ws281x;

/// A Python module implemented in Rust.
#[pymodule]
fn light_arrangements_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_test, m)?)?;
    m.add_function(wrap_pyfunction!(init_ws281x, m)?)?;
    m.add_class::<PyLightArrangement>()?;
    Ok(())
}
