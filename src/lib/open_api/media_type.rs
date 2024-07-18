use crate::lib::{
    context::Context,
    desc::Desc,
    open_api::{ref_or::RefOr, schema::Schema},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct MediaType {
    pub schema: RefOr<Schema>,
}

impl MediaType {
    pub(crate) fn of(desc: &Desc, context: &Context) -> MediaType {
        MediaType {
            schema: RefOr::Item(Schema::of_desc(desc, "MediaType", None, context)),
        }
    }
}
