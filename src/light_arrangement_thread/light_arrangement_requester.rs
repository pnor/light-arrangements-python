use std::sync::mpsc::channel;
use std::thread;

use light_arrangements::{ArrangementConfig, LightStripConfig, TestStrip, TestStripDisplayConfig};
use light_arrangements::{LightArrangement, LightArrangementError, LightStrip, Loc, RealStrip};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyResult;

use crate::types::PythonReturnColor;

use super::responses::Responses;
use super::{requests::Requests, LightArrangementThread};

impl<const N: usize> LightArrangementThread<N> {
    /// Spawns a new thread with the Light Arrangement. This object communicates with the thread to
    /// control the lights, to avoid moving th object between threads in the python runtime
    pub fn new<T: LightStrip + RealStrip>(
        strip_config: LightStripConfig,
        input_file: String,
    ) -> Result<Self, LightArrangementError> {
        let (request_sender, request_receiver) = channel();
        let (response_sender, response_receiver) = channel();

        thread::spawn(move || {
            let strip_result = T::new(strip_config);
            if let Err(error) = strip_result {
                eprintln!("Failed to create light strip: {}", error.reason());
                return;
            }

            let arrangement_config_result = ArrangementConfig::from_csv(&input_file);
            if let Err(error) = arrangement_config_result {
                eprintln!("Failed to create arrangement: {}", error.reason());
                return;
            }

            let light_arrangement_res =
                LightArrangement::new(strip_result.unwrap(), arrangement_config_result.unwrap());
            if let Ok(light_arrangement) = light_arrangement_res {
                Self::light_arrangement_thread(light_arrangement, request_receiver, response_sender)
            } else {
                eprintln!(
                    "Failed to create light arrangment: {}",
                    light_arrangement_res.err().unwrap().reason()
                );
            }
        });

        return Ok(Self {
            request_sender,
            response_receiver,
        });
    }

    /// Spawns a new thread with a Test Strip, returning the object to control it
    pub fn test(
        test_strip_config: TestStripDisplayConfig,
        input_file: String,
    ) -> Result<Self, LightArrangementError> {
        let (request_sender, request_receiver) = channel();
        let (response_sender, response_receiver) = channel();

        thread::spawn(move || {
            let arrangement_config_result = ArrangementConfig::from_csv(&input_file);
            if let Err(error) = arrangement_config_result {
                eprintln!("Failed to create arrangement: {}", error.reason());
                return;
            }

            let test_strip = TestStrip::new(
                &arrangement_config_result.as_ref().unwrap(),
                &test_strip_config,
            );

            let light_arrangement_res =
                LightArrangement::new(test_strip, arrangement_config_result.unwrap());
            if let Ok(light_arrangement) = light_arrangement_res {
                Self::light_arrangement_thread(light_arrangement, request_receiver, response_sender)
            } else {
                eprintln!(
                    "Failed to create light arrangment: {}",
                    light_arrangement_res.err().unwrap().reason()
                );
            }
        });

        return Ok(Self {
            request_sender,
            response_receiver,
        });
    }

    pub fn get_closest_polar(
        &self,
        loc: &Loc<N>,
        max_search_distance: f64,
    ) -> PyResult<Option<PythonReturnColor>> {
        let send_result = self
            .request_sender
            .send(Requests::GetClosestPolar(loc.clone(), max_search_distance));
        if let Err(err) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::ColorResponse(c)) => Ok(c),
            _ => Err(PyValueError::new_err(
                "Got wrong response internally from Light Arrangement thread",
            )),
        }
    }

    fn internal_response_error() -> PyErr {
        PyValueError::new_err("Got wrong response internally from Light Arrangement thread")
    }
}
