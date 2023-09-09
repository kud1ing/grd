#[macro_use]
extern crate log;

use grid_server_interface::grid_server_interface::{StatusRequest, StatusResponse};
use grid_server_interface::{
    ClientId, Grid, GridServer, Job, JobId, JobSubmitRequest, JobSubmitResponse,
    RegisterClientRequest, RegisterClientResponse, ResultFetchRequest, ResultFetchResponse,
    ResultSubmitRequest, ResultSubmitResponse, ServiceId, ServiceVersion,
    WorkerServerExchangeRequest, WorkerServerExchangeResponse,
};
use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::process::exit;
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

/// The grid server.
pub struct GridServerImpl {
    /// A map from the job IDs to the ID of the client the job was submitted from.
    client_id_per_job_id: Mutex<HashMap<JobId, ClientId>>,
    /// A map from the client ID to a map of the service version to its jobs.
    jobs_per_service_id_and_version:
        Mutex<HashMap<ServiceId, HashMap<ServiceVersion, VecDeque<Job>>>>,
    /// The next client ID.
    next_client_id: Mutex<ClientId>,
    /// The next job ID.
    next_job_id: Mutex<JobId>,
    /// Results per client ID.
    results_per_client_id: Mutex<HashMap<ClientId, Vec<grid_server_interface::Result>>>,
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

    ///
    fn add_result(&self, result: &grid_server_interface::Result) {
        let job_id = result.job_id;
        let maybe_client_id_for_job_id = self.client_id_per_job_id.lock().unwrap().remove(&job_id);

        // There is a client ID for the given job ID.
        if let Some(client_id_for_job_id) = maybe_client_id_for_job_id {
            info!("Accepting result for job with ID {job_id} from client {client_id_for_job_id}");

            // Collect the given result for the client ID.
            self.results_per_client_id
                .lock()
                .unwrap()
                .entry(client_id_for_job_id)
                .or_default()
                .push(result.clone());
        }
        // There is no client ID for the given job ID.
        else {
            warn!(
                "`add_result()`: there is no client ID for the job ID {}.",
                job_id
            );
        }
    }
}

/// The implementation of the server interface for the server.
#[tonic::async_trait]
impl Grid for GridServerImpl {
    async fn register_client(
        &self,
        _request: Request<RegisterClientRequest>,
    ) -> Result<Response<RegisterClientResponse>, Status> {
        // TODO: Grant or deny a client ID according to the request.
        warn!("TODO: `register_client()`: grant or deny a client ID according to the request.");

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
        let job_id;

        // Get the job ID.
        {
            // Get the next job ID.
            let mut next_job_id = self.next_job_id.lock().unwrap();

            // Copy the next job ID as the current job ID.
            job_id = *next_job_id;

            // Increase the next job ID.
            *next_job_id += 1;
        }

        let client_id = job_submit_request.get_ref().client_id;

        // Register the job ID with the given client ID.
        self.client_id_per_job_id
            .lock()
            .unwrap()
            .insert(job_id, client_id);

        info!("Accepting job with ID {job_id} from client {client_id}");

        // Collect the job from the given request.
        {
            let job_submit_request = job_submit_request.get_ref();

            let mut jobs_per_service_id_and_version =
                self.jobs_per_service_id_and_version.lock().unwrap();

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

        // Return the job ID.
        Ok(Response::new(JobSubmitResponse { job_id }))
    }

    async fn worker_server_exchange(
        &self,
        job_fetch_request: Request<WorkerServerExchangeRequest>,
    ) -> Result<Response<WorkerServerExchangeResponse>, Status> {
        let job_fetch_request = job_fetch_request.get_ref();

        {
            // There is a result from the worker.
            if let Some(result_from_worker) = &job_fetch_request.result_from_worker {
                self.add_result(&result_from_worker);
            }

            // Try to get jobs for the given request.
            if let Some(query_job_from_server) = &job_fetch_request.query_job_from_server {
                let mut jobs_per_service_id_and_version =
                    self.jobs_per_service_id_and_version.lock().unwrap();

                // There are jobs for the given service type.
                if let Some(jobs_per_service_version) =
                    jobs_per_service_id_and_version.get_mut(&query_job_from_server.service_id)
                {
                    // There are jobs for the given service version.
                    if let Some(jobs) =
                        jobs_per_service_version.get_mut(&query_job_from_server.service_version)
                    {
                        // Remove and return the first job.
                        if let Some(job) = jobs.pop_front() {
                            let job_id = job.job_id;

                            info!("Sending job with ID {job_id} to worker");

                            return Ok(Response::new(WorkerServerExchangeResponse {
                                job: Some(job),
                            }));
                        }
                    }
                }
            }
        }

        Ok(Response::new(WorkerServerExchangeResponse { job: None }))
    }

    async fn submit_result(
        &self,
        request: Request<ResultSubmitRequest>,
    ) -> Result<Response<ResultSubmitResponse>, Status> {
        let request = request.get_ref();

        // There is a result.
        if let Some(result) = &request.result {
            self.add_result(&result);
        }

        Ok(Response::new(ResultSubmitResponse {}))
    }

    async fn fetch_results(
        &self,
        result_fetch_request: Request<ResultFetchRequest>,
    ) -> Result<Response<ResultFetchResponse>, Status> {
        let result_fetch_request = result_fetch_request.get_ref();
        let client_id = result_fetch_request.client_id;

        let maybe_results = self
            .results_per_client_id
            .lock()
            .unwrap()
            .remove(&client_id);

        // There are results for the given client ID.
        if let Some(results) = maybe_results {
            info!("Sending results to client {client_id}");

            return Ok(Response::new(ResultFetchResponse { results }));
        }

        Ok(Response::new(ResultFetchResponse { results: vec![] }))
    }

    async fn get_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let mut status = HashMap::new();

        // TODO: jobs in queue
        // TODO: results in queue
        // TODO: connected clients
        //status.insert("", format!("", self.))

        Ok(Response::new(StatusResponse { status }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 2 {
        error!("Please pass the server address.");
        exit(-1);
    }

    // Construct the socket address the command line argument,
    let socket_address = command_line_arguments[1].parse()?;

    info!("Starting the server on \"{}\" ...", socket_address);

    Server::builder()
        .add_service(GridServer::new(GridServerImpl::new()))
        .serve(socket_address)
        .await?;

    Ok(())
}
