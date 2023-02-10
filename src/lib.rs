mod light_arrangement_thread;
mod util;

use led_arrangements::{
    self, strip_builder, ArrangementConfig, ArrangementConfigError, Color, LightConfig, LightStrip,
    Loc, TestStripDisplayConfig, Ws281xStrip,
};
use led_arrangements::{LightArrangement, TestStrip};
use light_arrangement_thread::LightArrangementThread;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use util::{color_to_tuple, vec_to_array};

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
    Test3D(LightArrangement<TestStrip, 3>),
    Test4D(LightArrangement<TestStrip, 4>),
    Ws281x1D(LightArrangementThread<1>),
    Ws281x2D(LightArrangementThread<2>),
    Ws281x3D(LightArrangementThread<3>),
    Ws281x4D(LightArrangementThread<4>),
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
                light_arr_enum: LightArrangementTypes::Test1d(test_arrangement),
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
        3 => {
            let test_arrangement = create_test_arrangement::<3>(
                TestStripDisplayConfig::new(sphere_size, camera_start, dimension_mask),
                input_file,
            )?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test2D(test_arrangement),
            });
        }
        4 => {
            let test_arrangement = create_test_arrangement::<4>(
                TestStripDisplayConfig::new(sphere_size, camera_start, dimension_mask),
                input_file,
            )?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test2D(test_arrangement),
            });
        }
        _ => Err(PyValueError::new_err(
            "Dimension number must be 1, 2, 3, or 4",
        )),
    }
}

fn init_ws281x(
    number_dimensions: usize,
    input_file: String,
    number_lights: i32,
    io_pin: i32,
) -> PyResult<PyLightArrangement> {
    match number_dimensions {
        1 => {
            let strip = strip_builder::ws281x(config);
            let light_arr_threading =
                LightArrangementThread::new(strip, input_file, number_lights, io_pin);
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x1D(light_arr_threading),
            });
        }
        2 => {
            let config = LightConfig::new(number_lights, io_pin);
            let strip = strip_builder::ws281x(config);
            let arrangement = to_pyresult(ArrangementConfig::<2>::from_csv(&input_file))?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x2D(strip),
            });
        }
        3 => {
            let config = LightConfig::new(number_lights, io_pin);
            let strip = strip_builder::ws281x(config);
            let arrangement = to_pyresult(ArrangementConfig::<3>::from_csv(&input_file))?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x3D(strip),
            });
        }
        4 => {
            let config = LightConfig::new(number_lights, io_pin);
            let strip = strip_builder::ws281x(config);
            let arrangement = to_pyresult(ArrangementConfig::<4>::from_csv(&input_file))?;
            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws2814Dx(strip),
            });
        }
        _ => Err(PyValueError::new_err(
            "Dimension number must be 1, 2, 3, or 4",
        )),
    }
}

#[pymethods]
impl PyLightArrangement {
    fn get_closest_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        max_search_distance: f64,
    ) -> Option<(u8, u8, u8)> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1d(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance);
                return color_to_tuple(opt_color);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance);
                return color_to_tuple(opt_color);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance);
                return color_to_tuple(opt_color);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance);
                return color_to_tuple(opt_color);
            }
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn led_arrangements_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_test, m)?)?;
    Ok(())
}
