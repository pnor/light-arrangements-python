use light_arrangements::Loc;
use pyo3::prelude::*;
use pyo3::PyResult;

use crate::types::PythonColor;
use crate::{types::PythonReturnColor, util::vec_to_array};

use super::{LightArrangementTypes, PyLightArrangement};

#[pymethods]
impl PyLightArrangement {
    pub fn get_closest_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        max_search_distance: f64,
    ) -> PyResult<Option<PythonReturnColor>> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
        }
    }

    pub fn set_closest_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        max_search_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
        }
    }

    pub fn show(&self) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let result = arr.show()?;
                return Ok(result);
            }
        }
    }
}