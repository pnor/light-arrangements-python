/// Given List of (<dimenstion_integer>, <LightArrangementTypes enum name>),
/// Implements every light arrangement method for every dimension-strip pair
#[macro_export]
macro_rules! impl_methods_for_dimensions {
    ( $( ($n:expr, $name:ident) ),* ) => {
        #[pymethods]
        impl PyLightArrangement {

            pub fn get_closest(
                &self,
                coordinate: Vec<f64>,
                max_search_distance: f64,
            ) -> PyResult<Option<PythonReturnColor>> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let loc = Loc::cartesian(vec_to_array::<$n>(coordinate)?);
                            let opt_color = arr.get_closest(&loc, max_search_distance)?;
                            return Ok(opt_color);
                        }
                    )*
                }
            }

            pub fn get_by_index(&self, index: usize) -> PyResult<PythonReturnColor> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let color = arr.get_by_index(index)?;
                            return Ok(color);
                        }
                    )*
                }
            }

            pub fn set_closest(
                &self,
                coordinate: Vec<f64>,
                max_search_distance: f64,
                color: PythonColor,
            ) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let loc = Loc::cartesian(vec_to_array::<$n>(coordinate)?);
                            let result = arr.set_closest(&loc, max_search_distance, color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn set_decreasing_intensity(
                &self,
                coordinate: Vec<f64>,
                set_distance: f64,
                color: PythonColor,
            ) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let loc = Loc::cartesian(vec_to_array::<$n>(coordinate)?);
                            let result = arr.set_decreasing_intensity(&loc, set_distance, color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn set_decreasing_intensity_merge(
                &self,
                coordinate: Vec<f64>,
                set_distance: f64,
                color: PythonColor,
            ) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let loc = Loc::cartesian(vec_to_array::<$n>(coordinate)?);
                            let result = arr.set_decreasing_intensity_merge(&loc, set_distance, color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn set_all_in_box(
                &self,
                loc1: Vec<f64>,
                loc2: Vec<f64>,
                color: PythonColor,
            ) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let loc1 = Loc::cartesian(vec_to_array(loc1)?);
                            let loc2 = Loc::cartesian(vec_to_array(loc2)?);
                            let result = arr.set_all_in_box(loc1, loc2, color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn set_all_in_radius(
                &self,
                coordinate: Vec<f64>,
                radius: f64,
                color: PythonColor,
            ) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let loc = Loc::cartesian(vec_to_array::<$n>(coordinate)?);
                            let result = arr.set_all_in_radius(loc, radius, color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn set_by_index(&self, index: usize, color: PythonColor) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let result = arr.set_by_index(index, color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn show(&self) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let result = arr.show()?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn fill(&self, color: PythonColor) -> PyResult<()> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            let result = arr.fill(color)?;
                            return Ok(result);
                        }
                    )*
                }
            }

            pub fn number_lights(&self) -> PyResult<i32> {
                match &self.light_arr_enum {
                    $(
                        LightArrangementTypes::$name(arr) => {
                            return Ok(arr.number_lights());
                        }
                    )*
                }
            }
        }
    };
}
