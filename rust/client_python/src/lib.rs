use client;
use client::connect_sync_grid_client;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use server_interface::{JobId, ServiceId, ServiceVersion};

#[pyclass]
struct Job {
    job_data: Vec<u8>,
    job_id: JobId,
}

///
fn job_from_interface_job(interface_job: server_interface::Job) -> Job {
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
fn interface_result_from_result(result: Result) -> server_interface::Result {
    server_interface::Result {
        job_id: result.job_id,
        result_data: result.result_data,
    }
}

///
fn result_from_interface_result(interface_result: server_interface::Result) -> Result {
    Result {
        job_id: interface_result.job_id,
        result_data: interface_result.result_data,
    }
}

// =================================================================================================

#[pyclass]
struct SyncGridClient {
    sync_grid_client: client::SyncGridClient,
}

#[pymethods]
impl SyncGridClient {
    #[new]
    fn new(host_name: &str, port: u32) -> PyResult<Self> {
        Ok(SyncGridClient {
            sync_grid_client: connect_sync_grid_client(host_name, port).map_err(|error| {
                PyTypeError::new_err(format!("Can not connect to the server: {}", error))
            })?,
        })
    }

    ///
    fn fetch_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
    ) -> PyResult<Option<Job>> {
        match self.sync_grid_client.fetch_job(service_id, service_version) {
            // TODO: Can we move the data instead of cloning?
            Ok(job_fetch_response) => Ok(job_fetch_response
                .get_ref()
                .clone()
                .job
                .map(job_from_interface_job)),
            Err(error) => Err(PyTypeError::new_err(format!(
                "Could not fetch a job from the server: {}",
                error
            ))),
        }
    }

    ///
    fn fetch_results(&mut self) -> PyResult<Vec<Result>> {
        match self.sync_grid_client.fetch_results() {
            // TODO: Can we move the data instead of cloning?
            Ok(result_fetch_response) => Ok(result_fetch_response
                .get_ref()
                .results
                .iter()
                .cloned()
                .map(result_from_interface_result)
                .collect()),
            Err(error) => Err(PyTypeError::new_err(format!(
                "Could not fetch result from the server: {}",
                error
            ))),
        }
    }

    ///
    fn submit_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        job_data: Vec<u8>,
    ) -> PyResult<JobId> {
        match self
            .sync_grid_client
            .submit_job(service_id, service_version, job_data)
        {
            Ok(job_submit_response) => Ok(job_submit_response.get_ref().job_id),
            Err(error) => Err(PyTypeError::new_err(format!(
                "Can not submit job to the server: {}",
                error
            ))),
        }
    }

    ///
    pub fn submit_result(&mut self, result: Result) -> PyResult<()> {
        match self
            .sync_grid_client
            .submit_result(interface_result_from_result(result))
        {
            Ok(_result_submit_response) => Ok(()),
            Err(error) => Err(PyTypeError::new_err(format!(
                "Can not submit result to the server: {}",
                error
            ))),
        }
    }
}

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
