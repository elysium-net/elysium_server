import subprocess
import os

os.environ["RT_WORKERS"] = "1"
os.environ["RT_STACK_SIZE"] = str(1024 * 1024)
os.environ["RT_MAX_BLOCKING_THREADS"] = "1"
subprocess.run(["cargo", "run"])
