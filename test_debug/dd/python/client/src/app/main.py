from trust.dev.service import DevService
from trust.table.service import TableService

dev_service = DevService()
table_service = TableService()

print(dev_service.get_health())
print(table_service.get_tables())