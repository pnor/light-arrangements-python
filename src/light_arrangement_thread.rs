use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;

use led_arrangements::{
    ArrangementConfig, ArrangementConfigError, LightArrangement, LightConfig, LightStrip, Loc,
};

use crate::util::{vec_to_array, vec_to_color};

/// Stored as Vector as that is the type python
type InputColor = Vec<u8>;
type OutputColor = (u8, u8, u8);

/// Object to interface with LightArrangements that are not Send. This constructs it on a seperate thread
/// and acesses it using thread channels. This is done to itnerface with the python API as it can
/// not send the state object across Python threads
pub struct LightArrangementThread<const N: usize> {
    request_sender: Sender<Requests<N>>,
    response_receiver: Receiver<Responses>,
}

/// Data sent between the threads to request information
enum Requests<const N: usize> {
    GetClosestPolar(f64, Vec<f64>, [f64; N], f64),
    SetClosestPolar(f64, Vec<f64>, [f64; N], f64, InputColor),
    Fill(Vec<u8>),
    Show,
    Quit,
}

/// Data sent between the threads to receive information
enum Responses {
    None,
    ColorResponse(OutputColor),
}

impl<const N: usize> LightArrangementThread<N> {
    pub fn new<T: LightStrip>(
        strip: T,
        input_file: String,
    ) -> Result<Self, ArrangementConfigError> {
        let arrangement = ArrangementConfig::<1>::from_csv(&input_file)?;
        let light_arrangement = LightArrangement::new(strip, arrangement)?;

        let (request_sender, request_receiver) = channel();
        let (response_sender, response_receiver) = channel();

        thread::spawn(move || {
            Self::light_arrangement_thread(light_arrangement, request_receiver, response_sender)
        });

        return Ok(Self {
            request_sender,
            response_receiver,
        });
    }

    fn light_arrangement_thread<T: LightStrip>(
        light_arrangement: LightArrangement<T, N>,
        request_receiver: Receiver<Requests<N>>,
        response_sender: Sender<Responses>,
    ) {
        let mut listening = true;

        while listening {
            let mut request = request_receiver.recv();
            match request {
                Err(RecvError) => {
                    println!("Got recv error when trying to read request response in other thread");
                    listening = false;
                }
                Ok(request) => match request {
                    Requests::Quit => {
                        listening = false;
                    }
                    Requests::GetClosestPolar(rho, angular_coords, center, max_search_distance) => {
                        let result = match light_arrangement.get_closest(
                            Loc::polar(rho, &angular_coords, center),
                            max_search_distance,
                        ) {
                            None => Responses::None,
                            Some(color) => {
                                Responses::ColorResponse((color.red, color.green, color.blue))
                            }
                        };
                        if response_sender.send(result).is_err() {
                            println!("Error sending result!");
                            listening = false;
                        }
                    }
                    Requests::SetClosestPolar(
                        rho,
                        angular_coords,
                        center,
                        max_search_distance,
                        color,
                    ) => {
                        light_arrangement.set_closest(
                            Loc::polar(rho, &angular_coords, center),
                            max_search_distance,
                            &vec_to_color(color),
                        );
                        response_sender.send(Responses::None);
                    }
                    Requests::Fill(color) => {
                        light_arrangement.fill(&vec_to_color(color));
                        response_sender.send(Responses::None);
                    }
                    Requests::Show => {
                        light_arrangement.show();
                        response_sender.send(Responses::None);
                    }
                },
            }
        }

        println!("Exiting LightArrangement thread");
    }
}

impl<const N: usize> Drop for LightArrangementThread<N> {
    fn drop(&mut self) {
        self.request_sender(Requests::Quit);
    }
}
