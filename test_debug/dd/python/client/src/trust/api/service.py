from trust.api import server_event
from trust import client

class ApiService:

    def subscribe_to_sse(self) -> server_event.ServerEventDto:
        query_params = {  }
        response = client.get("/sse".format(), params = query_params)
        return response

