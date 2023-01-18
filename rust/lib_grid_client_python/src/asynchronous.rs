use pyo3::prelude::*;

#[pyclass]
pub(crate) struct AsyncGridClient {
    // TODO
    //async_grid_client: client::AsyncGridClient,
}

#[pymethods]
impl AsyncGridClient {
    #[new]
    pub(crate) fn new(
        _py: Python,
        _server_address: &str,
        _port: u32,
        _client_name: String,
    ) -> PyResult<Self> {
        todo!()
        /*
        Ok(AsyncGridClient {
            async_grid_client: pyo3_asyncio::tokio::future_into_py_with_locals(
                connect_async_grid_client(server_address, client_name)
                    .await
                    .map_err(|error| {
                        PyTypeError::new_err(format!("Can not connect to the server: {}", error))
                    })?,
            ),
        })
        */
    }

    /*

    ///
    pub(crate) fn worker_server_exchange(
        &mut self,
        service_id: ServiceId,
        service_version: ServiceVersion,
    ) -> PyResult<Option<Job>> {
        match self.sync_grid_client.worker_server_exchange(service_id, service_version) {
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

     */
}
