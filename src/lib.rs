use light_arrangement_python_obj::PyLightArrangement;
use pyo3::prelude::*;
use pyo3::{types::PyModule, Python};

mod errors;
mod light_arrangement_python_obj;
mod light_arrangement_thread;
mod pyloc;
mod types;
mod util;

use light_arrangement_python_obj::init_test;
use light_arrangement_python_obj::init_ws281x;

use light_arrangements::Loc;
use pyo3::types::PyType;
use util::vec_to_array;

// Functions for constructing location coordinates
impl_pyloc_for_dimensions!(
    (1, PyLoc1, "Loc1"),
    (2, PyLoc2, "Loc2"),
    (3, PyLoc3, "Loc3"),
    (4, PyLoc4, "Loc4")
);

/// A Python module implemented in Rust.
#[pymodule]
fn light_arrangements_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_test, m)?)?;
    m.add_function(wrap_pyfunction!(init_ws281x, m)?)?;
    m.add_class::<PyLightArrangement>()?;

    macro_rules! add_pyloc_methods_to_module {
        ( $($name:ident),* ) => {
            $(
                m.add_class::<$name>()?;
            )*
        }
    }
    add_pyloc_methods_to_module!(PyLoc1, PyLoc2, PyLoc3, PyLoc4);

    Ok(())
}
