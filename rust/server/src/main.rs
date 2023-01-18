use server_interface::{
    ClientId, Grid, GridServer, Job, JobFetchRequest, JobFetchResponse, JobId, JobSubmitRequest,
    JobSubmitResponse, RegisterClientRequest, RegisterClientResponse, ResultFetchRequest,
    ResultFetchResponse, ResultSubmitRequest, ResultSubmitResponse, ServiceId, ServiceVersion,
};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

///
pub struct GridServerImpl {
    client_id_per_job_id: Mutex<HashMap<JobId, ClientId>>,
    jobs_per_service_id_and_version:
        Mutex<HashMap<ServiceId, HashMap<ServiceVersion, VecDeque<Job>>>>,
    next_client_id: Mutex<ClientId>,
    next_job_id: Mutex<JobId>,
    results_per_client_id: Mutex<HashMap<ClientId, Vec<server_interface::Result>>>,
}

impl GridServerImpl {
    ///
    fn new() -> Self {
        GridServerImpl {
            client_id_per_job_id: Mutex::new(HashMap::new()),
            jobs_per_service_id_and_version: Mutex::new(HashMap::new()),
            next_client_id: Mutex::new(0),
            next_job_id: Mutex::new(0),
            results_per_client_id: Mutex::new(HashMap::new()),
        }
    }
}

#[tonic::async_trait]
impl Grid for GridServerImpl {
    async fn register_client(
        &self,
        _request: Request<RegisterClientRequest>,
    ) -> Result<Response<RegisterClientResponse>, Status> {
        // TODO
        println!("TODO: `register_client()`: grant or deny a client ID according to the request");

        let mut next_client_id = self.next_client_id.lock().unwrap();

        // Get the current client ID.
        let client_id = *next_client_id;

        // Increase the next client ID.
        *next_client_id += 1;

        Ok(Response::new(RegisterClientResponse { client_id }))
    }

    async fn submit_job(
        &self,
        job_submit_request: Request<JobSubmitRequest>,
    ) -> Result<Response<JobSubmitResponse>, Status> {
        let mut next_job_id = self.next_job_id.lock().unwrap();

        // Get the current job ID.
        let job_id = *next_job_id;

        // Increase the next job ID.
        *next_job_id += 1;

        // Register the job ID with the given client ID.
        self.client_id_per_job_id
            .lock()
            .unwrap()
            .insert(job_id, job_submit_request.get_ref().client_id);

        // Collect the job from the request.
        {
            let mut jobs_per_service_id_and_version =
                self.jobs_per_service_id_and_version.lock().unwrap();

            let job_submit_request = job_submit_request.get_ref();

            // Add the given job data for the given service type and version.
            jobs_per_service_id_and_version
                .entry(job_submit_request.service_id)
                .or_default()
                .entry(job_submit_request.service_version)
                .or_default()
                .push_back(Job {
                    job_data: job_submit_request.job_data.clone(),
                    job_id,
                });
        }

        Ok(Response::new(JobSubmitResponse { job_id }))
    }

    async fn fetch_job(
        &self,
        job_fetch_request: Request<JobFetchRequest>,
    ) -> Result<Response<JobFetchResponse>, Status> {
        let job_fetch_request = job_fetch_request.get_ref();

        // Try to get jobs for the given request.
        {
            let mut jobs_per_service_id_and_version =
                self.jobs_per_service_id_and_version.lock().unwrap();

            // There are jobs for the given service type.
            if let Some(jobs_per_service_version) =
                jobs_per_service_id_and_version.get_mut(&job_fetch_request.service_id)
            {
                // There are jobs for the given service version.
                if let Some(jobs) =
                    jobs_per_service_version.get_mut(&job_fetch_request.service_version)
                {
                    // Remove and return the first job.
                    if let Some(job) = jobs.pop_front() {
                        return Ok(Response::new(JobFetchResponse { job: Some(job) }));
                    }
                }
            }
        }

        Ok(Response::new(JobFetchResponse { job: None }))
    }

    async fn submit_result(
        &self,
        result_submit_request: Request<ResultSubmitRequest>,
    ) -> Result<Response<ResultSubmitResponse>, Status> {
        let result_submit_request = result_submit_request.get_ref();

        // A result is given.
        if let Some(result) = &result_submit_request.result {
            // There is a client ID for the given job ID.
            if let Some(client_id) = self
                .client_id_per_job_id
                .lock()
                .unwrap()
                .get(&result.job_id)
            {
                // Collect the given result for the client ID.
                self.results_per_client_id
                    .lock()
                    .unwrap()
                    .entry(*client_id)
                    .or_default()
                    .push(result.clone());

                // TODO: Remove the job ID from `self.client_id_per_job_id`.
            }
            // There is no client ID for the given job ID.
            else {
                eprintln!(
                    "`submit_result()`: there is no client ID for the job ID {}.",
                    result.job_id
                );
            }
        }

        Ok(Response::new(ResultSubmitResponse {}))
    }

    async fn fetch_results(
        &self,
        result_fetch_request: Request<ResultFetchRequest>,
    ) -> Result<Response<ResultFetchResponse>, Status> {
        let result_fetch_request = result_fetch_request.get_ref();

        // There are results for the given client ID.
        if let Some(results) = self
            .results_per_client_id
            .lock()
            .unwrap()
            .remove(&result_fetch_request.client_id)
        {
            return Ok(Response::new(ResultFetchResponse {
                results: results.clone(),
            }));
        }

        Ok(Response::new(ResultFetchResponse { results: vec![] }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO
    println!("TODO: `main()`: get the port from the command line");
    let port = 50051;

    let socket_address = format!("[::1]:{}", port).parse()?;

    println!("Starting the server on port {} ...", port);

    Server::builder()
        .add_service(GridServer::new(GridServerImpl::new()))
        .serve(socket_address)
        .await?;

    Ok(())
}
