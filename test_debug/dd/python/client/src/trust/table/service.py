from trust.table import table
from trust.table import column_profile
from trust.table import typed_column
from trust.table import table_preview_response
from trust.table import table_id
from trust.table import column_id
from trust.table import upload_table_as_file
from trust.table import table_preview_request
from io import BufferedReader
from trust import client

class TableService:

    def get_tables(self) -> list[table.TableDto]:
        query_params = {  }
        response = client.get("/tables".format(), params = query_params)
        return response

    def upload_table_as_file(self, upload_table_as_file_dto: upload_table_as_file.UploadTableAsFileDto, file: BufferedReader) -> table_id.TableIdDto:
        query_params = {  }
        response = client.post("/tables/files".format(), params = query_params, data = upload_table_as_file_dto.dict(), files = {"file": file})
        return response

    def get_column_profiles(self, table_id: table_id.TableIdDto, bins_resolution: int, column_ids: list[column_id.ColumnIdDto]) -> list[column_profile.ColumnProfileDto]:
        query_params = { "binsResolution": bins_resolution, "columnIds": column_ids }
        response = client.get("/tables/{tableId}/column-profiles".format(tableId = table_id), params = query_params)
        return response

    def get_table_columns(self, table_id: table_id.TableIdDto) -> list[typed_column.TypedColumnDto]:
        query_params = {  }
        response = client.get("/tables/{tableId}/columns".format(tableId = table_id), params = query_params)
        return response

    def get_table_preview(self, table_preview_request_dto: table_preview_request.TablePreviewRequestDto, table_id: table_id.TableIdDto) -> table_preview_response.TablePreviewResponseDto:
        query_params = {  }
        response = client.post("/tables/{tableId}/preview".format(tableId = table_id), params = query_params, )
        return response

