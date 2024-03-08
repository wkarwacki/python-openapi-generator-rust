from trust.table import column_profile
from trust.table import table_preview_response
from trust.table import table_preview_request
from trust.table import upload_table_as_file
from trust.table import table_id
from trust.table import table
from trust.table import typed_column
from trust.table import column_id
from abc import ABC, abstractmethod

class TableService(ABC):
    

    @abstractmethod
    def get_tables(self) -> list[table.TableDto]:
        raise NotImplementedError
    

    @abstractmethod
    def upload_table_as_file(self, upload_table_as_file_dto: upload_table_as_file.UploadTableAsFileDto) -> table_id.TableIdDto:
        raise NotImplementedError
    

    @abstractmethod
    def get_column_profiles(self, table_id: table_id.TableIdDto, bins_resolution: int, column_ids: list[column_id.ColumnIdDto]) -> list[column_profile.ColumnProfileDto]:
        raise NotImplementedError
    

    @abstractmethod
    def get_table_columns(self, table_id: table_id.TableIdDto) -> list[typed_column.TypedColumnDto]:
        raise NotImplementedError
    

    @abstractmethod
    def get_table_preview(self, table_preview_request_dto: table_preview_request.TablePreviewRequestDto, table_id: table_id.TableIdDto) -> table_preview_response.TablePreviewResponseDto:
        raise NotImplementedError

