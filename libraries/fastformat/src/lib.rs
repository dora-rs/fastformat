pub use fastformat_datatypes::image;
pub use fastformat_datatypes::image::Image;

use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};

#[cfg(feature = "arrow")]
pub use fastformat_converter::arrow;

#[cfg(feature = "ndarray")]
pub use fastformat_converter::ndarray;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("hello fastformat".to_string())
}

#[pymodule]
fn fastformat(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, &m)?)?;
    m.add_wrapped(wrap_pymodule!(fastformat_datatypes::datatypes))?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
