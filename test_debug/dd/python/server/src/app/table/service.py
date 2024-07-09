from io import BufferedReader

from trust.table import table, table_id, upload_table_as_file, column_id, column_profile, typed_column, table_preview_request, \
    table_preview_response

from app.table.dto import Xxx
from trust.table.service import TableService


class TableServiceImpl(TableService):

    def get_tables(self) -> list[Xxx]:
        return []

    def upload_table_as_file(self, upload_table_as_file_dto: upload_table_as_file.UploadTableAsFileDto) -> table_id.TableIdDto:
        raise NotImplementedError

    def get_column_profiles(self, table_id: table_id.TableIdDto, bins_resolution: int, column_ids: list[column_id.ColumnIdDto]) -> list[column_profile.ColumnProfileDto]:
        raise NotImplementedError

    def get_table_columns(self, table_id: table_id.TableIdDto) -> list[typed_column.TypedColumnDto]:
        raise NotImplementedError

    def get_table_preview(self, table_preview_request_dto: table_preview_request.TablePreviewRequestDto, table_id: table_id.TableIdDto) -> table_preview_response.TablePreviewResponseDto:
        raise NotImplementedError
