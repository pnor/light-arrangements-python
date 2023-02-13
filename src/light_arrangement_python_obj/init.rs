use light_arrangements::LightStripConfig;
use light_arrangements::TestStripDisplayConfig;
use light_arrangements::Ws281xStrip;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::{errors::to_pyresult, light_arrangement_thread::LightArrangementThread};

use super::LightArrangementTypes;
use super::PyLightArrangement;

// TODO possible to make this not use any match on enum?
// just need to get the right strip constructed in the thread

#[pyfunction]
pub fn init_test(
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
            let light_arr_threading =
                LightArrangementThread::<1>::test(test_display_config, input_file)?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test1D(light_arr_threading),
            })
        }
        2 => {
            let light_arr_threading =
                LightArrangementThread::<2>::test(test_display_config, input_file)?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test2D(light_arr_threading),
            })
        }
        3 => {
            let light_arr_threading =
                LightArrangementThread::<3>::test(test_display_config, input_file)?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test3D(light_arr_threading),
            })
        }
        4 => {
            let light_arr_threading =
                LightArrangementThread::<4>::test(test_display_config, input_file)?;
            Ok(PyLightArrangement {
                light_arr_enum: LightArrangementTypes::Test4D(light_arr_threading),
            })
        }
        _ => Err(PyValueError::new_err(
            "Dimension number must be 1, 2, 3, or 4",
        )),
    };
    return pylight_arrangement;
}

#[pyfunction]
pub fn init_ws281x(
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
