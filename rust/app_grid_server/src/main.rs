mod client_information;

#[macro_use]
extern crate log;

use crate::client_information::ClientInformation;
use chrono::Utc;
use grid_server_interface::{
    ClientId, GridServer, GridServerServer, Job, JobId, RequestFromClientJobSubmit,
    RequestFromClientRegister, RequestFromClientResultFetch, RequestFromControllerStatusGet,
    RequestFromWorkerExchange, RequestFromWorkerResultSubmit, ResponseToClientJobSubmit,
    ResponseToClientRegister, ResponseToClientResultFetch, ResponseToControllerStatusGet,
    ResponseToWorkerExchange, ResponseToWorkerResultSubmit, ServiceId, ServiceVersion,
};
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::process::exit;
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

/// The grid server.
pub struct GridServerImpl {
    /// A map from the job IDs to the ID of the client the job was submitted from.
    client_id_per_job_id: Mutex<HashMap<JobId, ClientId>>,
    /// Information for every client, per client ID.
    client_information_per_client_id: Mutex<HashMap<ClientId, ClientInformation>>,
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
            client_information_per_client_id: Mutex::new(HashMap::new()),
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

    ///
    fn update_client_last_access_time(&self, _client_id: ClientId) {
        let _client_information_per_client_id =
            self.client_information_per_client_id.lock().unwrap();

        // TODO
        /*
        client_information_per_client_id
            .entry(client_id)
            .or_insert(ClientInformation {
                client_id: format!("{client_id}"),
                host_id: request.client_id.clone(),
                last_access: Utc::now(),
                user_id: request.user_id.clone(),
            });
         */
    }
}

/// The implementation of the server interface for the server.
#[tonic::async_trait]
impl GridServer for GridServerImpl {
    async fn client_fetch_results(
        &self,
        request: Request<RequestFromClientResultFetch>,
    ) -> Result<Response<ResponseToClientResultFetch>, Status> {
        let request = request.get_ref();
        let client_id = request.client_id;

        // Update the client's last access time.
        self.update_client_last_access_time(client_id);

        let maybe_results = self
            .results_per_client_id
            .lock()
            .unwrap()
            .remove(&client_id);

        // There are results for the given client ID.
        if let Some(results) = maybe_results {
            info!("Sending results to client {client_id}");

            return Ok(Response::new(ResponseToClientResultFetch { results }));
        }

        Ok(Response::new(ResponseToClientResultFetch {
            results: vec![],
        }))
    }

    async fn client_register(
        &self,
        request: Request<RequestFromClientRegister>,
    ) -> Result<Response<ResponseToClientRegister>, Status> {
        // TODO: Grant or deny a client ID according to the request.
        warn!("TODO: `client_register()`: grant or deny a client ID according to the request.");

        let mut next_client_id = self.next_client_id.lock().unwrap();

        // Get the current client ID.
        let client_id = *next_client_id;

        // Save client information.
        {
            let request = request.get_ref();
            let mut client_information_per_client_id =
                self.client_information_per_client_id.lock().unwrap();

            client_information_per_client_id
                .entry(client_id)
                .or_insert(ClientInformation {
                    client_description: request.client_description.clone(),
                    host_id: request.host_id.clone(),
                    last_access: Utc::now(),
                    user_id: request.user_id.clone(),
                });

            // client_information_per_client_id
        }

        // Increase the next client ID.
        *next_client_id += 1;

        Ok(Response::new(ResponseToClientRegister { client_id }))
    }

    async fn client_submit_job(
        &self,
        request: Request<RequestFromClientJobSubmit>,
    ) -> Result<Response<ResponseToClientJobSubmit>, Status> {
        let request = request.get_ref();
        let client_id = request.client_id;

        // Update the client's last access time.
        self.update_client_last_access_time(client_id);

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

        // Register the job ID with the given client ID.
        self.client_id_per_job_id
            .lock()
            .unwrap()
            .insert(job_id, client_id);

        info!("Accepting job with ID {job_id} from client {client_id}");

        // Collect the job from the given request.
        {
            let mut jobs_per_service_id_and_version =
                self.jobs_per_service_id_and_version.lock().unwrap();

            // Add the given job data for the given service type and version.
            jobs_per_service_id_and_version
                .entry(request.service_id)
                .or_default()
                .entry(request.service_version)
                .or_default()
                .push_back(Job {
                    job_data: request.job_data.clone(),
                    job_id,
                });
        }

        // Return the job ID.
        Ok(Response::new(ResponseToClientJobSubmit { job_id }))
    }

    async fn controller_get_status(
        &self,
        request: Request<RequestFromControllerStatusGet>,
    ) -> Result<Response<ResponseToControllerStatusGet>, Status> {
        let request = request.get_ref();
        let client_id = request.client_id;

        // Update the client's last access time.
        self.update_client_last_access_time(client_id);

        let mut status = HashMap::new();

        // Add the clients.
        status.insert(
            "clients".to_string(),
            format!(
                "{:?}",
                self.client_information_per_client_id.lock().unwrap()
            ),
        );
        // Add the queued jobs.
        status.insert(
            "jobs".to_string(),
            format!("{:?}", self.jobs_per_service_id_and_version.lock().unwrap()),
        );
        // Add the queued results.
        status.insert(
            "results".to_string(),
            format!("{:?}", self.results_per_client_id.lock().unwrap()),
        );

        Ok(Response::new(ResponseToControllerStatusGet {
            status: json!(status).to_string(),
        }))
    }

    async fn worker_server_exchange(
        &self,
        request: Request<RequestFromWorkerExchange>,
    ) -> Result<Response<ResponseToWorkerExchange>, Status> {
        let request = request.get_ref();
        let worker_client_id = request.client_id;

        // Update the grid worker client's last access time.
        self.update_client_last_access_time(worker_client_id);

        {
            // There is a result from the worker.
            if let Some(result_from_worker) = &request.result_from_worker {
                self.add_result(result_from_worker);
            }

            // Try to get jobs for the given request.
            if let Some(query_job_from_server) = &request.query_job_from_server {
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

                            return Ok(Response::new(ResponseToWorkerExchange { job: Some(job) }));
                        }
                    }
                }
            }
        }

        Ok(Response::new(ResponseToWorkerExchange { job: None }))
    }

    async fn worker_submit_result(
        &self,
        request: Request<RequestFromWorkerResultSubmit>,
    ) -> Result<Response<ResponseToWorkerResultSubmit>, Status> {
        let request = request.get_ref();
        let worker_client_id = request.client_id;

        // Update the client's last access time.
        self.update_client_last_access_time(worker_client_id);

        // There is a result.
        if let Some(result) = &request.result {
            self.add_result(result);
        }

        Ok(Response::new(ResponseToWorkerResultSubmit {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Get the given command line arguments.
    let command_line_arguments: Vec<_> = args().collect();

    // Too few command line arguments are given.
    if command_line_arguments.len() < 2 {
        error!("Please pass the server socket address.");
        exit(-1);
    }

    // Construct the socket address from the command line argument,
    let socket_address = command_line_arguments[1].parse()?;

    info!("Running the server on \"{}\" ...", socket_address);

    Server::builder()
        .add_service(GridServerServer::new(GridServerImpl::new()))
        .serve(socket_address)
        .await?;

    Ok(())
}
