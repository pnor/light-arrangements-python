use pyo3::prelude::*;

use crate::errors::to_pyresult;
use crate::light_arrangement_thread::LightArrangementThread;
use crate::types::PythonColor;
use crate::{types::PythonReturnColor, util::vec_to_array};

use light_arrangements::ColorOrder;
use light_arrangements::LightStripConfig;
use light_arrangements::Loc;
use light_arrangements::TestStripDisplayConfig;
use light_arrangements::Ws281xStrip;
use pyo3::exceptions::PyValueError;

mod init_macro;
mod methods_macro;

use crate::impl_init_test_for_dimensions;
use crate::impl_init_ws281x_for_dimensions;
use crate::impl_methods_for_dimensions;

#[pyclass]
pub struct PyLightArrangement {
    light_arr_enum: LightArrangementTypes,
}

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

impl_init_test_for_dimensions!((1, Test1D), (2, Test2D), (3, Test3D), (4, Test4D));
impl_init_ws281x_for_dimensions!((1, Ws281x1D), (2, Ws281x2D), (3, Ws281x3D), (4, Ws281x4D));

impl_methods_for_dimensions!(
    (1, Test1D),
    (2, Test2D),
    (3, Test3D),
    (4, Test4D),
    (1, Ws281x1D),
    (2, Ws281x2D),
    (3, Ws281x3D),
    (4, Ws281x4D)
);
