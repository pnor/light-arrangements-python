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

//static LIGHT_ARRANGEMENT_SINGLETON: Mutex<Arc<LightArrangementSingleton>> =
//    Mutex::new(LightArrangementSingleton::None);

// Let there be multiple
// but try to keep it in Rust

// == Library ==
// Setup
// - init arrangement for lights (arrangmeent file / array, strip_config)
// - init arrangement for test (arrangement, strip config, display config)
//
// spacial strip
// - [.] get closest(loc, dist) -> opt[color]
// - [.] set closest(loc, dist)
// - [.] set dec intensity(loc, dist, color)
// - [.] set dec intensity merge(loc, dist, color)
// - [.] set all in box(lower, upper, color)
// - [.] set all in radius(center, radius color)
// raw strip
// - get(index) -> Color
// - set(index, color)
//
// - [x] show()
// - [x] fill

/// A Python module implemented in Rust.
#[pymodule]
fn light_arrangements_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_test, m)?)?;
    m.add_function(wrap_pyfunction!(init_ws281x, m)?)?;
    m.add_class::<PyLightArrangement>()?;
    Ok(())
}
