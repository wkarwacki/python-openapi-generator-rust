import os
import pathlib
import shutil
import subprocess
from distutils.dir_util import copy_tree
from pydantic import BaseModel


class Gen(BaseModel):
    lang: str
    role: str
    out_dir: str

class Spec(BaseModel):
    name: str
    entrypoint: str
    params: list[str] = []
    
gens = [
    Gen(
        lang="python",
        role="client",
        out_dir="src"
    ),
    Gen(
        lang="python",
        role="server",
        out_dir="src"
    ),
]

specs = [
    Spec(
        name="openapi",
        entrypoint="api.yml"
    ),
    Spec(
        name="openapi_fastapi",
        entrypoint="api.yml",
        params = ["-l=tag"]
    ),
]

test_integration_path = pathlib.Path(__file__).parent.resolve()
os.chdir(test_integration_path.parent.parent)

shutil.rmtree(f"{test_integration_path}/run", ignore_errors=True)

clients = []
servers = []

for gen in gens:
    if gen.role == "client":
        clients.append(gen)
    elif gen.role == "server":
        servers.append(gen)
        
tests = os.environ["TESTS"].split(",")

for spec in specs:
    if spec.name in tests:
        trust_path = f"{test_integration_path}/run/{spec.name}/trust"
        subprocess.run(["cargo", "run", "from-open-api", f"{test_integration_path}/specs/{spec.name}/{spec.entrypoint}", trust_path, *spec.params])
        for gen in gens:
            run_path = f"{test_integration_path}/run/{spec.name}/{gen.lang}/{gen.role}"

            out_path = f"{run_path}/{gen.out_dir}"
            os.makedirs(out_path)

            for trust_file in os.listdir(trust_path):
                subprocess.run(["cargo", "run", "generate", gen.lang, gen.role, f"{trust_path}/{trust_file}", out_path])
    
            gen_path=f"{test_integration_path}/gens/{gen.lang}/{gen.role}"
            subprocess.run(f"{gen_path }/build.sh")
            copy_tree(gen_path, run_path)
    
        for server in servers:
            for client in clients:
                subprocess.run([f"{test_integration_path}/run.sh", f"run/{spec.name}/{server.lang}/{server.role}/run.sh", f"run/{spec.name}/{client.lang}/{client.role}/run.sh"])
