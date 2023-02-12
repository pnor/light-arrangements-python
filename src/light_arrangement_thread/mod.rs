mod light_arrangement_threading;
mod requests;
mod responses;

use pyo3::prelude::*;
use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};
use std::thread;

use light_arrangements::{
    ArrangementConfig, Color, LightArrangement, LightArrangementError, LightStrip,
    LightStripConfig, Loc, RealStrip,
};
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;

use crate::types::PythonReturnColor;
use crate::util::{vec_to_array, vec_to_color};

use self::requests::Requests;
use self::responses::Responses;

/// Object to interface with LightArrangements that are not Send. This constructs it on a seperate thread
/// and acesses it using thread channels. This is done to itnerface with the python API as it can
/// not send the state object across Python threads
pub struct LightArrangementThread<const N: usize> {
    request_sender: Sender<Requests<N>>,
    response_receiver: Receiver<Responses>,
}

impl<const N: usize> LightArrangementThread<N> {
    /// Spawns a new thread with the Light Arrangmenet. This object communicates with the thread to
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

impl<const N: usize> Drop for LightArrangementThread<N> {
    fn drop(&mut self) {
        self.request_sender.send(Requests::Quit);
    }
}
