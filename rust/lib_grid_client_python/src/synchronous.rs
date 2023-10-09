use crate::{
    interface_result_from_result, job_from_interface_job, result_from_interface_result, Job, Result,
};
use grid_client::connect_sync_grid_client;
use grid_server_interface::{JobId, ServiceId, ServiceVersion};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass]
pub(crate) struct SyncGridClient {
    sync_grid_client: grid_client::SyncGridClient,
}

#[pymethods]
impl SyncGridClient {
    #[new]
    pub(crate) fn new(server_address: &str, client_id: String) -> PyResult<Self> {
        Ok(SyncGridClient {
            sync_grid_client: connect_sync_grid_client(server_address, client_id).map_err(
                |error| PyTypeError::new_err(format!("Can not connect to the server: {}", error)),
            )?,
        })
    }

    ///
    pub(crate) fn client_fetch_results(&mut self) -> PyResult<Vec<Result>> {
        match self.sync_grid_client.client_fetch_results() {
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
    pub(crate) fn client_submit_job(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        job_data: Vec<u8>,
    ) -> PyResult<Option<JobId>> {
        match self
            .sync_grid_client
            .client_submit_job(service_id, service_version, job_data)
        {
            Ok(job_submit_response) => Ok(job_submit_response.get_ref().job_id),
            Err(error) => Err(PyTypeError::new_err(format!(
                "Can not submit job to the server: {}",
                error
            ))),
        }
    }

    ///
    pub(crate) fn worker_server_exchange(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
        result_from_worker: Option<Result>,
    ) -> PyResult<Option<Job>> {
        match self.sync_grid_client.worker_server_exchange(
            service_id,
            service_version,
            result_from_worker.map(interface_result_from_result),
        ) {
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
    pub(crate) fn worker_submit_result(&mut self, result: Result) -> PyResult<()> {
        match self
            .sync_grid_client
            .worker_submit_result(interface_result_from_result(result))
        {
            Ok(_) => Ok(()),
            Err(error) => Err(PyTypeError::new_err(format!(
                "Could not fetch a job from the server: {}",
                error
            ))),
        }
    }
}
