import subprocess

subprocess.run(["surreal",
                "start",
                "-u",
                "root",
                "-p",
                "root",
                "--log",
                "debug",
                "rocksdb://test_db"
                ])
