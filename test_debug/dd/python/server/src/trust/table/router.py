from trust.table import table_preview_request
from trust.table import upload_table_as_file
from trust.table import table_id
from trust.table import column_id
from .service import TableService

from fastapi import APIRouter, Depends, Path, Query
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

from typing import Annotated

table_router = APIRouter()


@table_router.get(
    "/tables"
)
def getTables(service: TableService = Depends(TableService)) -> JSONResponse:
    content = service.get_tables()
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@table_router.post(
    "/tables/files"
)
def uploadTableAsFile(request: upload_table_as_file.UploadTableAsFileDto = Depends(upload_table_as_file.UploadTableAsFileDto.of_form), service: TableService = Depends(TableService)) -> JSONResponse:
    content = service.upload_table_as_file(request)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@table_router.get(
    "/tables/{tableId}/column-profiles"
)
def getColumnProfiles(table_id: Annotated[table_id.TableIdDto, Path(alias = "tableId")], bins_resolution: Annotated[int, Query(alias = "binsResolution")], column_ids: Annotated[list[column_id.ColumnIdDto], Query(alias = "columnIds")], service: TableService = Depends(TableService)) -> JSONResponse:
    content = service.get_column_profiles(table_id, bins_resolution, column_ids)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@table_router.get(
    "/tables/{tableId}/columns"
)
def getTableColumns(table_id: Annotated[table_id.TableIdDto, Path(alias = "tableId")], service: TableService = Depends(TableService)) -> JSONResponse:
    content = service.get_table_columns(table_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@table_router.post(
    "/tables/{tableId}/preview"
)
def getTablePreview(request: table_preview_request.TablePreviewRequestDto, table_id: Annotated[table_id.TableIdDto, Path(alias = "tableId")], service: TableService = Depends(TableService)) -> JSONResponse:
    content = service.get_table_preview(request, table_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

