from fastapi import FastAPI

from trust.dev.router import dev_router
from trust.test.router import test_router
import uvicorn

from trust.dev.service import DevService
from trust.test.service import TestService

from app.dev.service import DevServiceImpl
from app.test.service import TestServiceImpl

app = FastAPI()

app.include_router(dev_router)
app.include_router(test_router)
app.dependency_overrides[DevService] = DevServiceImpl
app.dependency_overrides[TestService] = TestServiceImpl

def serve() -> None:
    uvicorn.run(
        "app.main:app",
        host="0.0.0.0",
        port=7999
    )

if __name__ == "__main__":
    serve()
