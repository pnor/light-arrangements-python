use light_arrangements::LightArrangementError;
use pyo3::{exceptions::PyValueError, prelude::*};

pub fn to_pyresult<T>(result: Result<T, LightArrangementError>) -> Result<T, PyErr> {
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(PyValueError::new_err(e.reason())),
    }
}
