from trust.test.service import TestService
from trust.test.test_id import TestIdDto
from trust.test.test_preview_request import TestPreviewRequestDto
from trust.test.test_preview_response import TestPreviewResponseDto


class TestServiceImpl(TestService):
    def get_test_preview(self, request: TestPreviewRequestDto,
                                test_id: TestIdDto) -> TestPreviewResponseDto:
        return TestPreviewResponseDto(
            last_row=8,
            row_circumference=17,
            row_data=[{"a": 1, "b": "c", "d": True}]
        )
