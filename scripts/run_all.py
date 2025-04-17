# code-trio/scripts/run_all.py
import subprocess
import os

def run_rust():
    subprocess.run(["cargo", "test"], cwd="packages/rust/linked_list")

def run_go():
    subprocess.run(["go", "test", "./linked_list"], cwd="packages/go")

def run_python():
    subprocess.run(["pytest", "linked_list/test_list.py"], cwd="packages/python")

if __name__ == "__main__":
    print("Running Rust packages...")
    run_rust()
    print("Running Go packages...")
    run_go()
    print("Running Python packages...")
    run_python()
