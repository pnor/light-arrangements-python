use crate::types::PythonReturnColor;

/// Data sent between the threads to receive information
pub enum Responses {
    InitOk,
    InitFailed,
    None,
    OptionColorResponse(Option<PythonReturnColor>),
    ColorResponse(PythonReturnColor),
    Error(String),
}
