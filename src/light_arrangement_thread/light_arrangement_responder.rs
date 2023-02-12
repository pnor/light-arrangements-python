/// Handles the work done by the thread that holds the LightArrangement Object.
/// Waits for requests from the main python thread, and sends back responses after doing the
/// computation work on this thread
use std::sync::mpsc::{Receiver, Sender};

use light_arrangements::{LightArrangement, LightStrip, RealStrip};

use crate::util::vec_to_color;

use super::{requests::Requests, responses::Responses, LightArrangementThread};

impl<const N: usize> LightArrangementThread<N> {
    pub fn light_arrangement_thread<T: LightStrip>(
        mut light_arrangement: LightArrangement<T, N>,
        request_receiver: Receiver<Requests<N>>,
        response_sender: Sender<Responses>,
    ) {
        let mut listening = true;

        while listening {
            let request = request_receiver.recv();
            match request {
                Err(RecvError) => {
                    eprintln!(
                        "Got recv error when trying to read request response in other thread"
                    );
                    listening = false;
                }
                Ok(request) => match request {
                    Requests::Quit => {
                        listening = false;
                    }
                    Requests::GetClosestPolar(loc, max_search_distance) => {
                        let result = match light_arrangement.get_closest(&loc, max_search_distance)
                        {
                            None => Responses::None,
                            Some(color) => {
                                Responses::ColorResponse(Some((color.red, color.green, color.blue)))
                            }
                        };
                        if response_sender.send(result).is_err() {
                            eprintln!("Error sending result!");
                            listening = false;
                        }
                    }
                    Requests::SetClosestPolar(loc, max_search_distance, color) => {
                        light_arrangement.set_closest(
                            &loc,
                            max_search_distance,
                            &vec_to_color(color),
                        );
                        if let Err(err) = response_sender.send(Responses::None) {
                            eprintln!("Error sending result!");
                            listening = false;
                        }
                    }
                    Requests::Fill(color) => {
                        light_arrangement.fill(&vec_to_color(color));
                        if let Err(err) = response_sender.send(Responses::None) {
                            eprintln!("Error sending result!");
                            listening = false;
                        }
                    }
                    Requests::Show => {
                        light_arrangement.show();
                        if let Err(err) = response_sender.send(Responses::None) {
                            eprintln!("Error sending result!");
                            listening = false;
                        }
                    }
                },
            }
        }

        println!("Exiting LightArrangement thread");
    }
}
