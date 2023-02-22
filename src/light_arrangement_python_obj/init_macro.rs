/// Given list of (<dimenstion_integer>, <LightArrangementTypes enum name for test>),
/// Implements the PyLightArrangement init function for TestStrips for every
/// integer dimension
#[macro_export]
macro_rules! impl_init_test_for_dimensions {
    ( $( ($n:expr, $name:ident) ),* ) => {
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
                $(
                    $n => {
                        let light_arr_threading =
                            LightArrangementThread::<$n>::test(test_display_config, input_file)?;
                        Ok(PyLightArrangement {
                            light_arr_enum: LightArrangementTypes::$name(light_arr_threading),
                        })
                    }
                )*
                x => Err(PyValueError::new_err(
                    format!("Dimension number could not be matched; no branch to handle dimension {}",x),
                )),
            };
            return pylight_arrangement;
        }
    };
}

/// Given list of (<dimenstion_integer>, <LightArrangementTypes enum name for ws281x>),
/// Implements the PyLightArrangement init function for TestStrips for every
/// integer dimension
#[macro_export]
macro_rules! impl_init_ws281x_for_dimensions {
    ( $( ($n:expr, $name:ident) ),* ) => {
        #[pyfunction]
        pub fn init_ws281x(
            number_dimensions: usize,
            input_file: String,
            number_lights: i32,
            io_pin: i32,
        ) -> PyResult<PyLightArrangement> {
            let strip_config = LightStripConfig::new(number_lights, io_pin);
            return match number_dimensions {
                $(
                    $n => {
                        let light_arr_threading = to_pyresult(
                            LightArrangementThread::<$n>::new::<Ws281xStrip>(strip_config, input_file),
                        )?;
                        Ok(PyLightArrangement {
                            light_arr_enum: LightArrangementTypes::$name(light_arr_threading),
                        })
                    }
                )*
                    x => Err(PyValueError::new_err(
                        format!("Dimension number could not be matched; no branch to handle dimension {}",x),
                    )),
            };
        }
    }
}
