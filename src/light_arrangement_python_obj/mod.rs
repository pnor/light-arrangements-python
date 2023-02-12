use pyo3::prelude::*;

use crate::light_arrangement_thread::LightArrangementThread;

mod init;
mod methods;

pub use init::init_test;
pub use init::init_ws281x;

pub enum LightArrangementTypes {
    Test1D(LightArrangementThread<1>),
    Test2D(LightArrangementThread<2>),
    Test3D(LightArrangementThread<3>),
    Test4D(LightArrangementThread<4>),
    Ws281x1D(LightArrangementThread<1>),
    Ws281x2D(LightArrangementThread<2>),
    Ws281x3D(LightArrangementThread<3>),
    Ws281x4D(LightArrangementThread<4>),
}

#[pyclass]
pub struct PyLightArrangement {
    light_arr_enum: LightArrangementTypes,
}
