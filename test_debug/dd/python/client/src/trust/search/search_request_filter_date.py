from trust.search import search_request_filter_date_operator
from trust.search import search_request_filter_abstraction

from typing import Any


class SearchRequestFilterDateDto(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):

    filter: dict[Any, Any] | None
    type: search_request_filter_date_operator.SearchRequestFilterDateOperatorDto
