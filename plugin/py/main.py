import argparse

import subprocess

from os import listdir
from pathlib import Path

parser = argparse.ArgumentParser(
    prog='TODO',
    description='TODO'
)

parser.add_argument('input')
parser.add_argument('output')
parser.add_argument('role') # TODO: enum
parser.add_argument('-t', '--templates')
parser.add_argument('-c', '--config')

args = parser.parse_args()

cmd = ["docker", "run", "--name", "trust"]

input = Path(args.input)
cmd.extend(["-v", f"{(input.parent if input.is_file() else input).absolute()}:/run/trust/api"])

if (args.templates):
    templates_path = "/usr/src/trust/templates"
    cmd.extend(["-v", f"{Path(args.templates).absolute()}:{templates_path}"])

if (args.config):
    config_path = "/usr/src/trust/gen-cfg.yml"
    cmd.extend(["-v", f"{args.config}:{config_path}"])

def __run(cmd: list[str], file:str, output: str) -> None:
    cmd_copy = cmd.copy()
    cmd_copy.extend(["trust", "generate", "python", args.role, f"/run/trust/api/{file}", "/run/trust/out"])

    if (args.templates):
        cmd_copy.extend(["-t", templates_path])

    if (args.config):
        cmd_copy.extend(["-c", config_path])

    print(cmd_copy)

    subprocess.run(cmd_copy)
    subprocess.run(["docker", "cp", "trust:/run/trust/out/.", output])
    subprocess.run(["docker", "rm", "trust"])

if input.is_file():
    __run(cmd, input.name, args.output)
else:
    for file in listdir(input):
        if Path(f"{input}/{file}").is_file():
            __run(cmd, file, args.output)

subprocess.run(["autoflake", "--recursive", "--in-place", "--remove-all-unused-imports", args.output])
subprocess.run(["black", "--exclude=pb2\.py|pb2_grpc\.py", args.output])
