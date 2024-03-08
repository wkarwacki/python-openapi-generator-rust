from trust.dev.service import DevService
from trust.table.service import TableService

dev_service = DevService()
table_sevice = TableService()

print(dev_service.get_health())
print(table_sevice.get_tables())