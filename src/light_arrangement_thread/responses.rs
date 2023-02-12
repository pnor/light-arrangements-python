use crate::types::PythonReturnColor;

/// Data sent between the threads to receive information
pub enum Responses {
    None,
    ColorResponse(Option<PythonReturnColor>),
}
