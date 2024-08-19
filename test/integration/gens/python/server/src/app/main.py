from fastapi import FastAPI

from trust.dev.router import dev_router
from trust.elephant.router import elephant_router
from trust.external_module.router import external_module_router
from trust.flaming.router import flaming_router
from trust.ibis.router import ibis_router
from trust.log.router import log_router
from trust.masterpiece.router import masterpiece_router
from trust.predator.router import predator_router
from trust.test.router import test_router
from trust.transformer.router import transformer_router
import uvicorn

app = FastAPI()

app.include_router(dev_router)
app.include_router(elephant_router)
app.include_router(external_module_router)
app.include_router(flaming_router)
app.include_router(ibis_router)
app.include_router(log_router)
app.include_router(masterpiece_router)
app.include_router(predator_router)
app.include_router(test_router)
app.include_router(transformer_router)

def serve() -> None:
    uvicorn.run(
        "app.main:app",
        host="0.0.0.0",
        port=7999
    )

if __name__ == "__main__":
    serve()
