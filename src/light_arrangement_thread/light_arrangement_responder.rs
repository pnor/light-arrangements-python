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
                    Requests::GetByIndex(index) => Self::thread_get_by_index(
                        &mut light_arrangement,
                        &response_sender,
                        index,
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
                    Requests::SetDecreasingIntensityMerge(loc, set_distance, color) => {
                        Self::thread_set_decreasing_intensity_merge(
                            &mut light_arrangement,
                            &response_sender,
                            &loc,
                            set_distance,
                            &color,
                            &mut listening,
                        )
                    }
                    Requests::SetBox(loc1, loc2, color) => Self::thread_set_all_in_box(
                        &mut light_arrangement,
                        &response_sender,
                        &loc1,
                        &loc2,
                        &color,
                        &mut listening,
                    ),
                    Requests::SetRadius(loc, radius, color) => Self::thread_set_all_in_radius(
                        &mut light_arrangement,
                        &response_sender,
                        &loc,
                        radius,
                        &color,
                        &mut listening,
                    ),
                    Requests::SetByIndex(index, color) => Self::thread_set_by_index(
                        &mut light_arrangement,
                        &response_sender,
                        index,
                        &color,
                        &mut listening,
                    ),
                    Requests::Fill(color) => Self::thread_fill(
                        &mut light_arrangement,
                        &response_sender,
                        &color,
                        &mut listening,
                    ),
                    Requests::Show => {
                        Self::thread_show(&mut light_arrangement, &response_sender, &mut listening)
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
            None => Responses::OptionColorResponse(None),
            Some(color) => {
                Responses::OptionColorResponse(Some((color.red, color.green, color.blue)))
            }
        };
        send_response_print_error(response_sender, result, listening);
    }

    fn thread_get_by_index<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        index: usize,
        listening: &mut bool,
    ) {
        if index >= light_arrangement.number_lights() {
            send_response_print_error(
                response_sender,
                Responses::Error(
                    format!(
                        "Index {} is out of bounds for light strip with {} lights",
                        index,
                        light_arrangement.number_lights()
                    )
                    .to_string(),
                ),
                listening,
            );
            return;
        }

        let color = light_arrangement.get_by_index(index);
        send_response_print_error(
            response_sender,
            Responses::ColorResponse((color.red, color.green, color.blue)),
            listening,
        );
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
        send_response_print_error(response_sender, Responses::None, listening);
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
        send_response_print_error(response_sender, Responses::None, listening);
    }

    fn thread_set_decreasing_intensity_merge<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        loc: &Loc<N>,
        set_distance: f64,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.set_decreasing_intensity_merge(&loc, set_distance, &vec_to_color(color));
        send_response_print_error(response_sender, Responses::None, listening);
    }

    fn thread_set_all_in_box<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        loc1: &Loc<N>,
        loc2: &Loc<N>,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.set_all_in_box(loc1, loc2, &vec_to_color(color));
        send_response_print_error(response_sender, Responses::None, listening);
    }

    fn thread_set_all_in_radius<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        loc: &Loc<N>,
        radius: f64,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.set_all_in_radius(loc, radius, &vec_to_color(color));
        send_response_print_error(response_sender, Responses::None, listening);
    }

    fn thread_set_by_index<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        index: usize,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        if index >= light_arrangement.number_lights() {
            send_response_print_error(
                response_sender,
                Responses::Error(
                    format!(
                        "Index {} is out of bounds for light strip with {} lights",
                        index,
                        light_arrangement.number_lights()
                    )
                    .to_string(),
                ),
                listening,
            );
            return;
        }

        light_arrangement.set_by_index(index, &vec_to_color(color));
        send_response_print_error(response_sender, Responses::None, listening);
    }

    fn thread_fill<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        color: &PythonColor,
        listening: &mut bool,
    ) {
        light_arrangement.fill(&vec_to_color(color));
        send_response_print_error(response_sender, Responses::None, listening);
    }

    fn thread_show<T: LightStrip>(
        light_arrangement: &mut LightArrangement<T, N>,
        response_sender: &Sender<Responses>,
        listening: &mut bool,
    ) {
        light_arrangement.show();
        send_response_print_error(response_sender, Responses::None, listening);
    }
}

/// Sends `response` through `sender`, printing the error and stopping the thread if it fails
fn send_response_print_error(
    sender: &Sender<Responses>,
    response: Responses,
    listening: &mut bool,
) {
    if sender.send(response).is_err() {
        eprintln!("Error sending result!");
        *listening = false;
    }
}
