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

    pub fn get_closest_cartesian(
        &self,
        coordinate: Vec<f64>,
        max_search_distance: f64,
    ) -> PyResult<Option<PythonReturnColor>> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<1>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<2>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<3>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<4>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<1>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<2>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<3>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<4>(coordinate));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
        }
    }

    pub fn get_closest_cylindrical(
        &self,
        radius: f64,
        theta: f64,
        coords: Vec<f64>,
        origin: Vec<f64>,
        max_search_distance: f64,
    ) -> PyResult<Option<PythonReturnColor>> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<1>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<2>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<3>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<4>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<1>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<2>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<3>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<4>(origin));
                let opt_color = arr.get_closest(&loc, max_search_distance)?;
                return Ok(opt_color);
            }
        }
    }

    pub fn get_by_index(&self, index: usize) -> PyResult<PythonReturnColor> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Test2D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Test3D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Test4D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let color = arr.get_by_index(index)?;
                return Ok(color);
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

    pub fn set_closest_cartesian(
        &self,
        coordinate: Vec<f64>,
        max_search_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<1>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<2>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<3>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<4>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<1>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<2>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<3>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::cartesian(vec_to_array::<4>(coordinate));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
        }
    }

    pub fn set_closest_cylindrical(
        &self,
        radius: f64,
        theta: f64,
        coords: Vec<f64>,
        origin: Vec<f64>,
        max_search_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<1>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<2>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<3>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<4>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<1>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<2>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<3>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::cylindrical(radius, theta, coords, &vec_to_array::<4>(origin));
                let result = arr.set_closest(&loc, max_search_distance, color)?;
                return Ok(result);
            }
        }
    }

    pub fn set_decreasing_intensity_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        set_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                return Ok(result);
            }
        }
    }

    pub fn set_decreasing_intensity_merge_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        set_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                return Ok(result);
            }
        }
    }

    pub fn set_all_in_box(
        &self,
        loc1: Vec<f64>,
        loc2: Vec<f64>,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc1 = Loc::cartesian(vec_to_array(loc1));
                let loc2 = Loc::cartesian(vec_to_array(loc2));
                let result = arr.set_all_in_box(loc1, loc2, color)?;
                return Ok(result);
            }
        }
    }

    pub fn set_all_in_radius_polar(
        &self,
        rho: f64,
        angular_coords: Vec<f64>,
        center: Vec<f64>,
        radius: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<1>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<2>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<3>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let loc = Loc::polar(rho, &angular_coords, &vec_to_array::<4>(center));
                let radius = radius;
                let result = arr.set_all_in_radius(loc, radius, color)?;
                return Ok(result);
            }
        }
    }

    pub fn set_by_index(&self, index: usize, color: PythonColor) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let result = arr.set_by_index(index, color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let result = arr.set_by_index(index, color)?;
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

    pub fn fill(&self, color: PythonColor) -> PyResult<()> {
        match &self.light_arr_enum {
            LightArrangementTypes::Test1D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test2D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test3D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Test4D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x1D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x2D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x3D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
            LightArrangementTypes::Ws281x4D(arr) => {
                let result = arr.fill(color)?;
                return Ok(result);
            }
        }
    }
}
