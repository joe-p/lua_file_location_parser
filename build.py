import sys
import os
import platform
import subprocess
import shutil

os.chdir(os.path.dirname(os.path.abspath(__file__)))


def run(command: str):
    result = subprocess.run(
        command.split(" "),
        text=True,
        stdout=None,  # Use None to inherit parent's stdout/stderr
        stderr=None,
    )

    if result.returncode != 0:
        raise Exception(f"{command} failed")


dest = os.path.join(
    "bin",
    (sys.platform.lower() + "-" + platform.machine()).lower(),
    "lua",
    "fetch_rs.so",
)

run("cargo build --release")
shutil.copy("target/release/libfetch_rs.dylib", dest)
