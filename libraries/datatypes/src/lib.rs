use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod bbox;
pub mod image;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("hello datatypes".to_string())
}

#[pymodule]
pub fn datatypes(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(image::image))?;
    m.add_wrapped(wrap_pymodule!(bbox::bbox))?;

    m.add_function(wrap_pyfunction!(hello, &m)?)?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
