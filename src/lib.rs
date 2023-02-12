mod errors;
mod light_arrangement_thread;
mod types;
mod util;

use errors::to_pyresult;
use light_arrangement_thread::LightArrangementThread;
use light_arrangements::{
    self, ArrangementConfig, Color, LightArrangementError, LightStrip, LightStripConfig, Loc,
    TestStripDisplayConfig, Ws281xStrip,
};
use light_arrangements::{LightArrangement, TestStrip};
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

#[pyclass]
struct PyLightArrangement {
    light_arr_enum: LightArrangementTypes,
}

enum LightArrangementTypes {
    Test1D(LightArrangementThread<1>),
    Test2D(LightArrangementThread<2>),
    Test3D(LightArrangementThread<3>),
    Test4D(LightArrangementThread<4>),
    Ws281x1D(LightArrangementThread<1>),
    Ws281x2D(LightArrangementThread<2>),
    Ws281x3D(LightArrangementThread<3>),
    Ws281x4D(LightArrangementThread<4>),
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
    let test_display_config =
        TestStripDisplayConfig::new(sphere_size, camera_start, dimension_mask);
    let pylight_arrangement = match number_dimensions {
        1 => {
            let light_arr_threading = to_pyresult(LightArrangementThread::<1>::test(
                test_display_config,
                input_file,
            ))?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x1D(light_arr_threading),
            })
        }
        2 => {
            let light_arr_threading = to_pyresult(LightArrangementThread::<2>::test(
                test_display_config,
                input_file,
            ))?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x2D(light_arr_threading),
            })
        }
        3 => {
            let light_arr_threading = to_pyresult(LightArrangementThread::<3>::test(
                test_display_config,
                input_file,
            ))?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x3D(light_arr_threading),
            })
        }
        4 => {
            let light_arr_threading = to_pyresult(LightArrangementThread::<4>::test(
                test_display_config,
                input_file,
            ))?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x4D(light_arr_threading),
            })
        }
        _ => Err(PyValueError::new_err(
            "Dimension number must be 1, 2, 3, or 4",
        )),
    };
    return pylight_arrangement;
}

#[pyfunction]
fn init_ws281x(
    number_dimensions: usize,
    input_file: String,
    number_lights: i32,
    io_pin: i32,
) -> PyResult<PyLightArrangement> {
    let strip_config = LightStripConfig::new(number_lights, io_pin);
    return match number_dimensions {
        1 => {
            let light_arr_threading = to_pyresult(
                LightArrangementThread::<1>::new::<Ws281xStrip>(strip_config, input_file),
            )?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x1D(light_arr_threading),
            })
        }
        2 => {
            let light_arr_threading = to_pyresult(
                LightArrangementThread::<2>::new::<Ws281xStrip>(strip_config, input_file),
            )?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x2D(light_arr_threading),
            })
        }
        3 => {
            let light_arr_threading = to_pyresult(
                LightArrangementThread::<3>::new::<Ws281xStrip>(strip_config, input_file),
            )?;

            return Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x3D(light_arr_threading),
            });
        }
        4 => {
            let light_arr_threading = to_pyresult(
                LightArrangementThread::<4>::new::<Ws281xStrip>(strip_config, input_file),
            )?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Ws281x4D(light_arr_threading),
            })
        }
        _ => Err(PyValueError::new_err(
            "Dimension number must be 1, 2, 3, or 4",
        )),
    };
}

#[pymethods]
impl PyLightArrangement {
    fn get_closest_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        max_search_distance: f64,
    ) -> PyResult<Option<(u8, u8, u8)>> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let opt_color = arr.get_closest_polar(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn light_arrangements_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyLightArrangement>()?;
    Ok(())
}
