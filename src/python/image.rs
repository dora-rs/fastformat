#![allow(clippy::borrow_deref_ref)]

use std::sync::Arc;

use crate::datatypes::Image;
use arrow::array::{Array, ArrayData, UInt8Array, UnionArray};
use arrow::pyarrow::PyArrowType;
use pyo3::exceptions::PyValueError;
use pyo3::ffi::Py_None;
use pyo3::types::{PyList, PyNone};
use pyo3::{exceptions::PyTypeError, prelude::*};
use pyo3::{wrap_pyfunction, wrap_pymodule};

#[pyclass]
pub struct PyArrowData(Option<PyArrowType<ArrayData>>);

#[pyclass]
pub struct PyImage(Option<Image<'static>>);

#[pymethods]
impl PyImage {
    pub fn name(&self) -> Option<&str> {
        self.0.as_ref().unwrap().name.as_deref()
    }

    pub fn width(&self) -> u32 {
        self.0.as_ref().unwrap().width
    }

    pub fn height(&self) -> u32 {
        self.0.as_ref().unwrap().height
    }

    pub fn as_ptr(&self) -> u64 {
        self.0.as_ref().unwrap().data.as_ptr() as u64
    }

    pub fn into_rgb8(&mut self) -> PyResult<PyImage> {
        match self.0.take().unwrap().into_rgb8() {
            Ok(image) => Ok(PyImage(Some(image))),
            Err(e) => Err(PyErr::new::<PyTypeError, _>(e.to_string())),
        }
    }

    pub fn into_bgr8(&mut self) -> PyResult<PyImage> {
        match self.0.take().unwrap().into_bgr8() {
            Ok(image) => Ok(PyImage(Some(image))),
            Err(e) => Err(PyErr::new::<PyTypeError, _>(e.to_string())),
        }
    }

    pub fn into_arrow(&mut self) -> PyResult<PyArrowData> {
        match self.0.take().unwrap().into_arrow() {
            Ok(array) => Ok(PyArrowData(Some(PyArrowType(array)))),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }
}

#[pyfunction]
pub fn new_rgb8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> PyResult<PyImage> {
    match Image::new_rgb8(data, width, height, name) {
        Ok(image) => Ok(PyImage(Some(image))),
        Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
    }
}

#[pyfunction]
pub fn new_bgr8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> PyResult<PyImage> {
    match Image::new_bgr8(data, width, height, name) {
        Ok(image) => Ok(PyImage(Some(image))),
        Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
    }
}

#[pyfunction]
pub fn new_gray8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> PyResult<PyImage> {
    match Image::new_gray8(data, width, height, name) {
        Ok(image) => Ok(PyImage(Some(image))),
        Err(e) => Err(PyErr::new::<PyTypeError, _>(e.to_string())),
    }
}

#[pyfunction]
pub fn from_arrow(array: &mut PyArrowData) -> PyResult<PyImage> {
    let array_data = array.0.take().unwrap().0;

    match Image::from_arrow(array_data) {
        Ok(image) => Ok(PyImage(Some(image))),
        Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
    }
}

#[pymodule]
fn image(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyImage>()?;
    m.add_function(wrap_pyfunction!(new_rgb8, &m)?)?;
    m.add_function(wrap_pyfunction!(new_bgr8, &m)?)?;
    m.add_function(wrap_pyfunction!(new_gray8, &m)?)?;
    m.add_function(wrap_pyfunction!(from_arrow, &m)?)?;

    Ok(())
}

#[pymodule]
fn fastformat(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(image))?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
