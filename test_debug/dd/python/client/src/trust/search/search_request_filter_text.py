from trust.search import search_request_filter_text_operator
from trust.search import search_request_filter_abstraction

from typing import Any


class SearchRequestFilterTextDto(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):

    filter: dict[Any, Any] | None
    type: search_request_filter_text_operator.SearchRequestFilterTextOperatorDto
