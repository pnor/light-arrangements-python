mod light_arrangement_requester;
mod light_arrangement_responder;
mod requests;
mod responses;

use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};

use self::{requests::Requests, responses::Responses};

/// Object to interface with LightArrangements that are not Send. This constructs it on a seperate thread
/// and acesses it using thread channels. This is done to itnerface with the python API as it can
/// not send the state object across Python threads
pub struct LightArrangementThread<const N: usize> {
    request_sender: Sender<Requests<N>>,
    response_receiver: Receiver<Responses>,
}

impl<const N: usize> Drop for LightArrangementThread<N> {
    fn drop(&mut self) {
        let response = self.request_sender.send(Requests::Quit);
        match response {
            Ok(_) => println!("Succesfully dropped the light arrangement thread"),
            Err(_) => println!("Failed to send the quit message to the light arrangement thread"),
        };
    }
}
