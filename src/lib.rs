use pyo3::prelude::*;

#[pyfunction]
fn hello() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[pymodule]
fn dndlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
