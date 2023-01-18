use crate::{
    interface_result_from_result, job_from_interface_job, result_from_interface_result, Job, Result,
};
use client;
use client::connect_sync_grid_client;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use server_interface::{JobId, ServiceId, ServiceVersion};

#[pyclass]
pub(crate) struct SyncGridClient {
    sync_grid_client: client::SyncGridClient,
}

#[pymethods]
impl SyncGridClient {
    #[new]
    pub(crate) fn new(host_name: &str, port: u32, client_name: String) -> PyResult<Self> {
        Ok(SyncGridClient {
            sync_grid_client: connect_sync_grid_client(host_name, port, client_name).map_err(
                |error| PyTypeError::new_err(format!("Can not connect to the server: {}", error)),
            )?,
        })
    }

    ///
    pub(crate) fn fetch_job(
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
    pub(crate) fn fetch_results(&mut self) -> PyResult<Vec<Result>> {
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
    pub(crate) fn submit_job(
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
    pub(crate) fn submit_result(&mut self, result: Result) -> PyResult<()> {
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
