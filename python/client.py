"""
A grid client.
"""
import pickle
import time
from grid import SyncGridClient

# Connect to the server.
grid_client = SyncGridClient("[::1]", 50051)

jobs_per_id = dict()

for i in range(0, 100):
    # Create a job.
    job = {"a": 1.0, "b": 2.0 * i}

    # Submit the job.
    service_id = 0
    service_version = 0
    job_id = grid_client.submit_job(service_id, service_version, pickle.dumps(job))

    jobs_per_id[job_id] = job

print(f"Created {len(jobs_per_id)} jobs")
print("Waiting for results ...")

while jobs_per_id:
    # Fetch results
    results = grid_client.fetch_results()

    if not results:
        time.sleep(1.0)
        continue

    for result in results:
        print(result.job_id, pickle.loads(result.result_data))

print("Done.")
