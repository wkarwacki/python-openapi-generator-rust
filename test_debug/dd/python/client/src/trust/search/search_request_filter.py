from trust.search import search_request_filter_text_operator
from trust.search import search_request_filter_date_operator
from trust.search import search_request_filter_abstraction
from trust.search import search_request_filter_number_operator

from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class SearchRequestFilterDtoBase(Dto):
    pass
class SearchRequestFilterDtoDate(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):

    filter: dict[Any, Any] | None
    type: search_request_filter_date_operator.SearchRequestFilterDateOperatorDto

    filter_type: Literal["date"]


class SearchRequestFilterDtoNumber(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):

    filter: dict[Any, Any] | None
    filter_to: dict[Any, Any] | None
    type: search_request_filter_number_operator.SearchRequestFilterNumberOperatorDto

    filter_type: Literal["number"]


class SearchRequestFilterDtoSet(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):
    

    


    values: list[dict[Any, Any] | None]

    filter_type: Literal["set"]


class SearchRequestFilterDtoText(search_request_filter_abstraction.SearchRequestFilterAbstractionDto):

    filter: dict[Any, Any] | None
    type: search_request_filter_text_operator.SearchRequestFilterTextOperatorDto

    filter_type: Literal["text"]



SearchRequestFilterDto = Annotated[
     SearchRequestFilterDtoDate | SearchRequestFilterDtoNumber | SearchRequestFilterDtoSet | SearchRequestFilterDtoText,
     Field(discriminator="filter_type")
 ]
 
