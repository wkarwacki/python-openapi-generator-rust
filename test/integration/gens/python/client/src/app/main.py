from trust.dev.service import DevService
from trust.test.service import TestService

from trust.test.test_preview_request import TestPreviewRequestDto
from trust.test.test_preview_response import TestPreviewResponseDto

dev_service = DevService()

print(dev_service.get_health())

test_service = TestService()
assert test_service.get_test_preview(TestPreviewRequestDto(
    start_row=1,
    end_row=3,
    sort_masterpiece=[],
    filter_masterpiece={}
), "test_id") == TestPreviewResponseDto(
    last_row=8,
    row_circumference=17,
    row_data=[{"a": 1, "b": "c", "d": True}]
)
