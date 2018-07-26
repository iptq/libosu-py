#![feature(use_extern_macros, specialization)]

extern crate libosu;
extern crate pyo3;

use std::path::PathBuf;

use libosu::OszDeserializer;
use pyo3::prelude::*;

#[pyclass]
pub struct Beatmap {
    inner: libosu::Beatmap,
}

#[pymethods]
impl Beatmap {
    #[new]
    fn __new__(obj: &PyRawObject, contents: String) -> PyResult<()> {
        let beatmap;
        match libosu::Beatmap::deserialize_osz(contents) {
            Ok(value) => beatmap = value,
            Err(error) => {
                return Err(PyErr::new::<exc::ValueError,_>(format!(
                    "failed to parse beatmap: {}",
                    error
                )))
            }
        }
        obj.init(|t| Beatmap { inner: beatmap })
    }
}

#[pymodinit]
fn libosu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Beatmap>()?;
    Ok(())
}
