use led_arrangements::{
    self, ArrangementConfig, ArrangementConfigError, LightConfig, LightStrip,
    TestStripDisplayConfig, Ws281xStrip,
};
use led_arrangements::{LightArrangement, TestStrip};
use pyo3::exceptions::PyValueError;
use pyo3::impl_::pyfunction;
use pyo3::prelude::*;

// Let there be multiple
// but try to keep it in Rust

// == Library ==
// Setup
// - init arrangement for lights (arrangmeent file / array, strip_config)
// - init arrangement for test (arrangement, strip config, display config)
//
// spacial strip
// - get closest(loc, dist) -> opt[color]
// - set closest(loc, dist)
// - set dec intensity(loc, dist, color)
// - set dec intensity merge(loc, dist, color)
// - set all in box(lower, upper, color)
// - set all in radius(center, radius color)
// raw strip
// - get(index) -> Color
// - set(index, color)
//
// - show()
// - fill

enum LightArrangementTypes {
    Test1D(LightArrangement<TestStrip, 1>),
    Test2D(LightArrangement<TestStrip, 2>),
    None,
}

#[pyclass]
struct PyLightArrangement {
    light_arr_enum: LightArrangementTypes,
}

fn to_pyresult<T>(result: Result<T, ArrangementConfigError>) -> Result<T, PyErr> {
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(PyValueError::new_err(e.reason())),
    }
}

fn create_test_arrangement<const N: usize>(
    display_config: TestStripDisplayConfig,
    input_file: String,
) -> Result<LightArrangement<TestStrip, N>, PyErr> {
    let arrangement_config = to_pyresult(ArrangementConfig::<N>::from_csv(&input_file))?;
    let strip = TestStrip::new(&arrangement_config, &display_config);
    let light_arrangement = to_pyresult(LightArrangement::new(strip, arrangement_config))?;
    return Ok(light_arrangement);
}

// ===== Exported Functions to python module

#[pyfunction]
fn init_test(
    number_dimensions: usize,
    input_file: String,
    sphere_size: f32,
    camera_start: (f32, f32, f32),
    dimension_mask: [u8; 3],
) -> PyResult<PyLightArrangement> {
    match number_dimensions {
        1 => {
            let test_arrangement = create_test_arrangement::<1>(
                TestStripDisplayConfig::new(sphere_size, camera_start, dimension_mask),
                input_file,
            )?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test1D(test_arrangement),
            });
        }
        2 => {
            let test_arrangement = create_test_arrangement::<2>(
                TestStripDisplayConfig::new(sphere_size, camera_start, dimension_mask),
                input_file,
            )?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test2D(test_arrangement),
            });
        }
        _ => Err(PyValueError::new_err("Dimension number must be 1, 2, or 3")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn led_arrangements_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_test, m)?)?;
    Ok(())
}
