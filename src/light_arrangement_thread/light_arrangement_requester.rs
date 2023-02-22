use std::sync::mpsc::channel;
use std::thread;

use light_arrangements::{ArrangementConfig, LightStripConfig, TestStrip, TestStripDisplayConfig};
use light_arrangements::{LightArrangement, LightArrangementError, LightStrip, Loc, RealStrip};
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;

use crate::types::{PythonColor, PythonReturnColor};

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
        let number_lights = strip_config.number_lights;

        thread::spawn(move || {
            let strip_result = T::new(strip_config);
            if let Err(error) = strip_result {
                eprintln!("Failed to create light strip: {}", error.reason());
                return;
            }

            let arrangement_config_result = ArrangementConfig::from_csv(&input_file, 100);
            if let Err(error) = arrangement_config_result {
                eprintln!("Failed to create arrangement: {}", error.reason());
                return;
            }

            let light_arrangement_res =
                LightArrangement::new(strip_result.unwrap(), arrangement_config_result.unwrap());
            if let Ok(light_arrangement) = light_arrangement_res {
                Self::light_arrangement_thread(
                    light_arrangement,
                    request_receiver,
                    response_sender,
                );
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
            number_lights,
        });
    }

    /// Spawns a new thread with a Test Strip, returning the object to control it
    pub fn test(test_strip_config: TestStripDisplayConfig, input_file: String) -> PyResult<Self> {
        let (request_sender, request_receiver) = channel();
        let (response_sender, response_receiver) = channel();

        let arrangement_config_result = ArrangementConfig::from_csv(&input_file, 100);
        if let Err(_) = arrangement_config_result {
            return Err(PyValueError::new_err(
                "Failed to create arrangement from csv file",
            ));
        }

        let number_lights = arrangement_config_result
            .as_ref()
            .unwrap()
            .light_locations
            .len() as i32;

        thread::spawn(move || {
            // Move the arrangement into the new thread
            let arrangement_config_result = arrangement_config_result;

            let test_strip = TestStrip::new(
                &arrangement_config_result.as_ref().unwrap(),
                &test_strip_config,
            );

            let light_arrangement_res =
                LightArrangement::new(test_strip, arrangement_config_result.unwrap());

            if let Ok(light_arrangement) = light_arrangement_res {
                if response_sender.send(Responses::InitOk).is_err() {
                    eprintln!(
                        "Failed to send back to main thread that light arrangmeent succesfully
                started"
                    );
                }
                Self::light_arrangement_thread(light_arrangement, request_receiver, response_sender)
            } else {
                eprintln!(
                    "Failed to create light arrangment: {}",
                    light_arrangement_res.err().unwrap().reason()
                );
                if response_sender.send(Responses::InitFailed).is_err() {
                    eprintln!(
                        "Failed to send back to main thread that light arrangement thread
                failed to start"
                    );
                }
                return;
            }
        });

        let init_response = response_receiver.recv();
        return match init_response {
            Ok(Responses::InitOk) => Ok(Self {
                request_sender,
                response_receiver,
                number_lights,
            }),
            Ok(Responses::InitFailed) => Err(PyValueError::new_err(
                "Failed to start light arrangement thread!",
            )),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Failed to start light arrangement thread; Internally returned wrong response",
            )),
            _ => Err(PyValueError::new_err(
                "Failed to start light thread due to receive error",
            )),
        };
    }

    pub fn get_closest(
        &self,
        loc: &Loc<N>,
        max_search_distance: f64,
    ) -> PyResult<Option<PythonReturnColor>> {
        let send_result = self
            .request_sender
            .send(Requests::GetClosest(loc.clone(), max_search_distance));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::OptionColorResponse(c)) => Ok(c),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            _ => Err(PyValueError::new_err(
                "Got wrong response internally from Light Arrangement thread",
            )),
        }
    }

    pub fn get_by_index(&self, index: usize) -> PyResult<PythonReturnColor> {
        let send_result = self.request_sender.send(Requests::GetByIndex(index));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::ColorResponse(c)) => Ok(c),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            _ => Err(PyValueError::new_err(
                "Got wrong response internally from Light Arrangement thread",
            )),
        }
    }

    pub fn set_closest(
        &self,
        loc: &Loc<N>,
        max_search_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        let send_result = self.request_sender.send(Requests::SetClosest(
            loc.clone(),
            max_search_distance,
            color,
        ));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            Err(_) => {
                return Err(PyValueError::new_err(
                    "Failed to receive response from Light Arrangement thread",
                ));
            }
        }
    }

    pub fn set_decreasing_intensity(
        &self,
        loc: &Loc<N>,
        set_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        let send_result = self.request_sender.send(Requests::SetDecreasingIntensity(
            loc.clone(),
            set_distance,
            color,
        ));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            Err(_) => {
                return Err(PyValueError::new_err(
                    "Failed to receive response from Light Arrangement thread",
                ));
            }
        }
    }

    pub fn set_decreasing_intensity_merge(
        &self,
        loc: &Loc<N>,
        set_distance: f64,
        color: PythonColor,
    ) -> PyResult<()> {
        let send_result = self
            .request_sender
            .send(Requests::SetDecreasingIntensityMerge(
                loc.clone(),
                set_distance,
                color,
            ));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            Err(_) => {
                return Err(PyValueError::new_err(
                    "Failed to receive response from Light Arrangement thread",
                ));
            }
        }
    }

    pub fn set_all_in_box(&self, loc1: Loc<N>, loc2: Loc<N>, color: PythonColor) -> PyResult<()> {
        let send_result =
            self.request_sender
                .send(Requests::SetBox(loc1.clone(), loc2.clone(), color));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            Err(_) => {
                return Err(PyValueError::new_err(
                    "Failed to receive response from Light Arrangement thread",
                ));
            }
        }
    }

    pub fn set_all_in_radius(&self, loc: Loc<N>, radius: f64, color: PythonColor) -> PyResult<()> {
        let send_result = self
            .request_sender
            .send(Requests::SetRadius(loc.clone(), radius, color));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            Err(_) => {
                return Err(PyValueError::new_err(
                    "Failed to receive response from Light Arrangement thread",
                ));
            }
        }
    }

    pub fn set_by_index(&self, index: usize, color: PythonColor) -> PyResult<()> {
        let send_result = self.request_sender.send(Requests::SetByIndex(index, color));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            Err(_) => {
                return Err(PyValueError::new_err(
                    "Failed to receive response from Light Arrangement thread",
                ));
            }
        }
    }

    pub fn fill(&self, color: PythonColor) -> PyResult<()> {
        let send_result = self.request_sender.send(Requests::Fill(color));
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            _ => Err(PyValueError::new_err(
                "Got wrong response internally from Light Arrangement thread",
            )),
        }
    }

    pub fn show(&self) -> PyResult<()> {
        let send_result = self.request_sender.send(Requests::Show);
        if let Err(_) = send_result {
            return Err(PyValueError::new_err("Unable to send request"));
        }

        match self.response_receiver.recv() {
            Ok(Responses::None) => Ok(()),
            Ok(Responses::Error(reason)) => Err(PyValueError::new_err(reason)),
            Ok(_) => Err(PyValueError::new_err(
                "Expected None response internally but got value",
            )),
            _ => Err(PyValueError::new_err(
                "Got wrong response internally from Light Arrangement thread",
            )),
        }
    }

    pub fn number_lights(&self) -> i32 {
        self.number_lights
    }
}
