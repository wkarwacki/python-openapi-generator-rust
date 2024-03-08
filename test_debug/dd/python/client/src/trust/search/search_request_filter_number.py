from trust.search import search_request_filter_abstraction
from trust.search import search_request_filter_number_operator

from typing import Any


class SearchRequestFilterNumberDto(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):

    filter: dict[Any, Any] | None
    filter_to: dict[Any, Any] | None
    type: search_request_filter_number_operator.SearchRequestFilterNumberOperatorDto
