use light_arrangements::Loc;

use crate::types::PythonColor;

/// Data sent between the threads to request information
pub enum Requests<const N: usize> {
    GetClosestPolar(Loc<N>, f64),
    SetClosestPolar(Loc<N>, f64, PythonColor),
    Fill(Vec<u8>),
    Show,
    Quit,
}
