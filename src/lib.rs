extern crate pyo3;

extern crate chrono;

use chrono::prelude::*;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::*;


#[pyfunction]
fn parse(_py: Python, str_datetime: &str, fmt: &str) -> PyResult<Py<PyDateTime>> {
    // Call chrono and ask it to parse the datetime for us
    let chrono_dt = Utc.datetime_from_str(str_datetime, fmt);

    match chrono_dt {
        // In case everything's fine, get Rust datetime out of the result and transform
        // it into a Python datetime.
        Ok(dt) => {
            let microsecond = dt.nanosecond() / 1000;
            // Build a Python datetime
            let py_dt = PyDateTime::new(
                _py,
                dt.year(),
                dt.month() as u8,
                dt.day() as u8,
                dt.hour() as u8,
                dt.minute() as u8,
                dt.second() as u8,
                microsecond,
                None,
            )?;
            Ok(py_dt.into())
        }
        // In case chrono couldn't parse a datetime, raise a ValueError with chrono's error message.
        // Because there are no exceptions in Rust, we return a PyValueError instance here.
        // By convention, it will make PyO3 wrapper raise an exception in Python interpreter.
        // https://pyo3.rs/v0.12.4/exception.html
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

#[pymodule]
#[pyo3(name="_dtparse")]
fn dtparse(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
