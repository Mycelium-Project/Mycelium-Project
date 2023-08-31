
use pyo3::prelude::*;
use tracing::Level;

// import traceback, threading
// call_frame = traceback.extract_stack()[-2]
// print(call_frame.lineno)
// print(call_frame.filename)
// print("py:"+threading.current_thread().name)

pub fn tracing_log(py: Python, level: Level, msg: String) -> PyResult<()> {
    let traceback = py.import("traceback")?;
    let threading = py.import("threading")?;
    let call_frame = traceback
        .getattr("extract_stack")?
        .call0()?
        .get_item(-2)?;
    // let name = call_frame.getattr("name")?.extract::<String>()?;
    let lineno = call_frame.getattr("lineno")?.extract::<String>()?;
    let filename = call_frame.getattr("filename")?.extract::<String>()?;
    let mut thread = threading
        .getattr("current_thread")?
        .call0()?
        .getattr("name")?
        .extract::<String>()?;
    thread = format!("py:{}", thread);

    match level {
        Level::TRACE => tracing::trace!(source = "scripting", message = msg, line = lineno, file = filename, thread = thread),
        Level::DEBUG => tracing::debug!(source = "scripting", message = msg, line = lineno, file = filename, thread = thread),
        Level::INFO => tracing::info!(source = "scripting", message = msg, line = lineno, file = filename, thread = thread),
        Level::WARN => tracing::warn!(source = "scripting", message = msg, line = lineno, file = filename, thread = thread),
        Level::ERROR => tracing::error!(source = "scripting", message = msg, line = lineno, file = filename, thread = thread),
    }

    Ok(())
}

#[pyfunction]
pub fn trace(py: Python, msg: String) -> PyResult<()> {
    tracing_log(py, Level::TRACE, msg)
}

#[pyfunction]
pub fn debug(py: Python, msg: String) -> PyResult<()> {
    tracing_log(py, Level::DEBUG, msg)
}

#[pyfunction]
pub fn info(py: Python, msg: String) -> PyResult<()> {
    tracing_log(py, Level::INFO, msg)
}

#[pyfunction]
pub fn warn(py: Python, msg: String) -> PyResult<()> {
    tracing_log(py, Level::WARN, msg)
}

#[pyfunction]
pub fn error(py: Python, msg: String) -> PyResult<()> {
    tracing_log(py, Level::ERROR, msg)
}

#[pymodule]
fn logging(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(trace, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(warn, m)?)?;
    m.add_function(wrap_pyfunction!(error, m)?)?;

    Ok(())
}