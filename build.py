import toml
import subprocess
import os

with open('Cargo.toml', 'r') as f:
    cargo_toml = toml.load(f)
major, minor, patch = cargo_toml['package']['version'].split('.')

for feature in ["zh", "en"]:
    binary_name = f"pvz_interception_calculator_v{major}_{minor}_{patch}_win_{feature}.exe"
    build_command = ["cargo", "build", "--release", f"--features={feature}"]
    subprocess.run(build_command, check=True)
    old_name = "./target/release/pvz_interception_calculator.exe"
    new_name = f"./target/release/{binary_name}"
    if os.path.isfile(new_name):
        os.remove(new_name)
    os.rename(old_name, new_name)
