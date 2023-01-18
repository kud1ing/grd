"""
A grid worker.
"""
from grid import Result, SyncGridClient
import pickle
import time

# Connect to the server.
grid_client = SyncGridClient("[::1]", 50051)

print("Processing jobs ...")

service_id = 0
service_version = 0

while True:
    # Fetch a job.
    job = grid_client.fetch_job(service_id, service_version)

    if not job:
        time.sleep(1.0)
        continue

    job_data = pickle.loads(job.job_data)

    # Process the job.
    sum = job_data["a"] + job_data["b"]

    # Submit the result.
    grid_client.submit_result(Result(job.job_id, pickle.dumps(sum)))

print("Done.")
