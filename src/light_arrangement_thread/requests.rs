use light_arrangements::Loc;

use crate::types::PythonColor;

/// Data sent between the threads to request information
pub enum Requests<const N: usize> {
    GetClosest(Loc<N>, f64),
    SetClosest(Loc<N>, f64, PythonColor),
    SetDecreasingIntensity(Loc<N>, f64, PythonColor),
    SetDecreasingIntensityMerge(Loc<N>, f64, PythonColor),
    SetBox(Loc<N>, Loc<N>, PythonColor),
    SetRadius(Loc<N>, f64, PythonColor),
    GetByIndex(usize),
    SetByIndex(usize),
    Fill(Vec<u8>),
    Show,
    Quit,
}
