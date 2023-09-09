mod asynchronous;
mod synchronous;

use crate::synchronous::SyncGridClient;
use grid_server_interface::JobId;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
struct Job {
    job_data: Vec<u8>,
    job_id: JobId,
}

///
fn job_from_interface_job(interface_job: grid_server_interface::Job) -> Job {
    Job {
        job_data: interface_job.job_data,
        job_id: interface_job.job_id,
    }
}

#[pymethods]
impl Job {
    #[getter]
    fn job_data<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        Ok(PyBytes::new(py, &self.job_data))
    }

    #[getter]
    fn job_id(&self) -> PyResult<JobId> {
        Ok(self.job_id)
    }
}

// =================================================================================================

#[pyclass]
#[derive(Clone)]
struct Result {
    job_id: JobId,
    result_data: Vec<u8>,
}

#[pymethods]
impl Result {
    #[new]
    fn new(job_id: JobId, result_data: Vec<u8>) -> Self {
        Result {
            job_id,
            result_data,
        }
    }

    #[getter]
    fn job_id(&self) -> PyResult<JobId> {
        Ok(self.job_id)
    }

    #[getter]
    fn result_data(&self) -> PyResult<&[u8]> {
        Ok(&self.result_data)
    }
}

///
fn interface_result_from_result(result: Result) -> grid_server_interface::Result {
    grid_server_interface::Result {
        job_id: result.job_id,
        result_data: result.result_data,
    }
}

///
fn result_from_interface_result(interface_result: grid_server_interface::Result) -> Result {
    Result {
        job_id: interface_result.job_id,
        result_data: interface_result.result_data,
    }
}

// =================================================================================================

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn grid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Job>()?;
    m.add_class::<Result>()?;
    m.add_class::<SyncGridClient>()?;
    Ok(())
}
