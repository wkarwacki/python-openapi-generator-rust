from fastapi import FastAPI

from trust.dev.router import dev_router
import uvicorn

from trust.dev.service import DevService

from app.dev.service import DevServiceImpl

app = FastAPI()

app.include_router(dev_router)
app.dependency_overrides[DevService] = DevServiceImpl

def serve() -> None:
    uvicorn.run(
        "app.main:app",
        host="0.0.0.0",
        port=8000
    )

if __name__ == "__main__":
    serve()
