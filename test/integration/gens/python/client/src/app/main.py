from trust.dev.service import DevService
from trust.test.service import TestService

from trust.test.test_preview_request import TestPreviewRequestDto
from trust.test.test_preview_response import TestPreviewResponseDto

dev_service = DevService()

print(dev_service.get_health())

test_service = TestService()
assert test_service.get_test_preview(TestPreviewRequestDto.get(), "test_id") == TestPreviewResponseDto.get()
