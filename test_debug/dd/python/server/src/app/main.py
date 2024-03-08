from fastapi import FastAPI

from app.table.service import TableServiceImpl
from trust.dev.router import dev_router
import uvicorn

from trust.dev.service import DevService
from trust.table.router import table_router

from trust.table.service import TableService

from app.dev.service import DevServiceImpl

app = FastAPI()

app.include_router(dev_router)
app.dependency_overrides[DevService] = DevServiceImpl

app.include_router(table_router)
app.dependency_overrides[TableService] = TableServiceImpl

def serve() -> None:
    uvicorn.run(
        "app.main:app",
        host="0.0.0.0",
        port=8000
    )

if __name__ == "__main__":
    serve()
