import os
import pathlib
import shutil
import subprocess
from pydantic import BaseModel


class Gen(BaseModel):
    lang: str
    role: str
    out_dir: str

class Test(BaseModel):
    name: str
    entrypoint: str
    dir_input: bool
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

tests = [
    Test(
        name="openapi",
        entrypoint="api.yml",
        dir_input=True
    ),
    Test(
        name="openapi_fastapi",
        entrypoint="api.yml",
        dir_input=False,
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
        

for test in tests:
    trust_path = f"{test_integration_path}/run/{test.name}/trust"
    subprocess.run(["cargo", "run", "from-open-api", f"{test_integration_path}/tests/{test.name}/{test.entrypoint}", trust_path, *test.params])
    for gen in gens:
        run_path = f"{test_integration_path}/run/{test.name}/{gen.lang}/{gen.role}"

        out_path = f"{run_path}/{gen.out_dir}"
        os.makedirs(out_path)

        gen_path=f"{test_integration_path}/gens/{gen.lang}/{gen.role}"

        if test.dir_input:
            subprocess.run(["cargo", "run", "generate", gen.lang, gen.role, trust_path, out_path, "-c", f"{gen_path}/trust-cfg.yml"])
        else:
            for trust_file in os.listdir(trust_path):
                subprocess.run(["cargo", "run", "generate", gen.lang, gen.role, f"{trust_path}/{trust_file}", out_path, "-c", f"{gen_path}/trust-cfg.yml"])

        subprocess.run(f"{gen_path }/build.sh")
        shutil.copytree(gen_path, run_path, dirs_exist_ok=True)

    for server in servers:
        for client in clients:
            subprocess.run([f"{test_integration_path}/run.sh", f"run/{test.name}/{server.lang}/{server.role}/run.sh", f"run/{test.name}/{client.lang}/{client.role}/run.sh"], check=True)
