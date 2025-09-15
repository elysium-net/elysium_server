import subprocess

subprocess.run(["mailhog", "-smtp-bind-addr", "0.0.0.0:25"])
