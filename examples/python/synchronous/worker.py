"""
A grid worker.
"""
from grid import Result, SyncGridClient
import pickle
import time

# Connect to the server.
grid_client = SyncGridClient("[::1]:50051", "worker")

print("Processing jobs ...")

service_id = 0
service_version = 0

result = None

while True:
    if result is not None:
        result = pickle.dumps(result)

    # Fetch a job and maybe send a result.
    job = grid_client.worker_server_exchange(service_id, service_version, result)

    if not job:
        time.sleep(1.0)
        continue

    job_data = pickle.loads(job.job_data)

    # Process the job.
    result = job_data["a"] + job_data["b"]

print("Done.")
