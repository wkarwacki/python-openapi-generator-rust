
from trust import client

class DevService:

    def get_health(self) -> None:
        query_params = {  }
        client.get("/health".format(), params = query_params)

