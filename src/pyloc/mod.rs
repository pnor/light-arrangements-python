/// Implement functions to convert coordinates to a common cartesian system
/// Takes input:
/// (n, rust_name, python_name)
/// Where
/// - n: the number of dimensions of output coordinate
/// - rust_name: Name of Rust struct created
/// - python_name: Name of class in python
///
#[macro_export]
macro_rules! impl_pyloc_for_dimensions {

    ( $( ($x:expr, $name:ident, $python_name:expr) ),* ) => {
        $(
            #[pyclass(name=$python_name)]
            pub struct $name {
                pub loc: Loc<$x>,
            }

            #[pymethods]
            impl $name {
                #[classmethod]
                pub fn cartesian(_cls: &PyType, coordinate: Vec<f64>) -> Vec<f64> {
                        Loc::cartesian(vec_to_array::<$x>(coordinate)).coords.to_vec()
                }

                #[classmethod]
                pub fn polar(
                    _cls: &PyType,
                    rho: f64,
                    angular_coords: Vec<f64>,
                    center: Vec<f64>,
                ) -> Vec<f64> {
                    Loc::polar(rho, &angular_coords, &vec_to_array::<$x>(center)).coords.to_vec()
                }

                #[classmethod]
                pub fn cylindrical(
                    _cls: &PyType,
                    radius: f64,
                    theta: f64,
                    coords: Vec<f64>,
                    origin: Vec<f64>,
                ) -> Vec<f64> {
                        Loc::cylindrical(radius, theta, coords, &vec_to_array::<$x>(origin)).coords.to_vec()
                }
            }
        )*
    };
}
