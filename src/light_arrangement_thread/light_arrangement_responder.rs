/// Handles the work done by the thread that holds the LightArrangement Object.
/// Waits for requests from the main python thread, and sends back responses after doing the
/// computation work on this thread
use std::sync::mpsc::{Receiver, Sender};

use light_arrangements::{LightArrangement, LightStrip, Loc};

use crate::{types::PythonColor, util::vec_to_color};

use super::{requests::Requests, responses::Responses, LightArrangementThread};

impl<const N: usize> LightArrangementThread<N> {
    pub fn light_arrangement_thread<T: LightStrip>(
        mut light_arrangement: LightArrangement<T, N>,
        request_receiver: Receiver<Requests<N>>,
        response_sender: Sender<Responses>,
    ) {
        let mut listening = true;

        while listening {
            println!("listening...");
            let request = request_receiver.recv();
            match request {
                Err(_) => {
                    eprintln!(
                        "Got recv error when trying to read request response in other thread"
                    );
                    listening = false;
                }
                Ok(request) => match request {
                    Requests::Quit => {
                        listening = false;
                    }
                    Requests::GetClosest(loc, max_search_distance) => Self::thread_get_closest(
                        &light_arrangement,
                        &response_sender,
                        &loc,
                        max_search_distance,
                        &mut listening,
                    ),
                    Requests::SetClosest(loc, max_search_distance, color) => {
                        Self::thread_set_closest(
                            &mut light_arrangement,
                            &response_sender,
                            &loc,
                            max_search_distance,
                            &color,
                            &mut listening,
                        )
                    }
                    Requests::SetDecreasingIntensity(loc, set_distance, color) => {
                        Self::thread_set_decreasing_intensity(
                            &mut light_arrangement,
                            &response_sender,
                            &loc,
                            set_distance,
                            &color,
                            &mut listening,
                        )
                    }
                    Requests::Fill(color) => Self::thread_fill(
                        &mut light_arrangement,
                        &response_sender,
                        &color,
                        &mut listening,
                    ),
                    Requests::Show => {
                        Self::thread_show(&mut light_arrangement, &response_sender, &mut listening)
                    }
                    _ => {
                        eprintln!("unimplemented");
                        listening = false;
                    }
                },
            }
        }

        println!("Exiting LightArrangement thread");
    }

    fn thread_get_closest<T: LightStrip>(
        light_arrangement: &LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        loc: &Loc<N>,
        max_search_distance: f64,
        listening: &mut bool,
    ) {
        let result = match light_arrangement.get_closest(&loc, max_search_distance) {
            None => Responses::ColorResponse(None),
            Some(color) => Responses::ColorResponse(Some((color.red, color.green, color.blue))),
        };
        if response_sender.send(result).is_err() {
            eprintln!("Error sending result!");
            *listening = false;
        }
    }

    fn thread_set_closest<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        loc: &Loc<N>,
        max_search_distance: f64,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.set_closest(&loc, max_search_distance, &vec_to_color(color));
        if let Err(_) = response_sender.send(Responses::None) {
            eprintln!("Error sending result!");
            *listening = false;
        }
    }

    fn thread_set_decreasing_intensity<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        loc: &Loc<N>,
        set_distance: f64,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.set_decreasing_intensity(&loc, set_distance, &vec_to_color(color));
        if let Err(_) = response_sender.send(Responses::None) {
            eprintln!("Error sending result!");
            *listening = false;
        }
    }

    fn thread_fill<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.fill(&vec_to_color(color));
        if let Err(_) = response_sender.send(Responses::None) {
            eprintln!("Error sending result!");
            *listening = false;
        }
    }

    fn thread_show<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        listening: &mut bool,
    ) {
        light_arrangement.show();
        if let Err(_) = response_sender.send(Responses::None) {
            eprintln!("Error sending result!");
            *listening = false;
        }
    }
}
