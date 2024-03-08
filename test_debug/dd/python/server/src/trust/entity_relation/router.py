from trust.entity_relation import generate_entity_relations_request
from .service import EntityRelationService

from fastapi import APIRouter, Depends
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse


entity_relation_router = APIRouter()


@entity_relation_router.post(
    "/entity-relations/generations"
)
def generateEntityRelations(request: generate_entity_relations_request.GenerateEntityRelationsRequestDto, service: EntityRelationService = Depends(EntityRelationService)) -> JSONResponse:
    content = service.generate_entity_relations(request)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

