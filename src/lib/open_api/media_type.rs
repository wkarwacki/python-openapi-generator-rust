use crate::lib::context::Context;
use crate::lib::desc::Desc;
use crate::lib::open_api::ref_or::RefOr;
use crate::lib::open_api::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MediaType {
    pub schema: RefOr<Schema>,
}

impl MediaType {
    pub fn of(desc: Desc, context: &Context) -> MediaType {
        MediaType {
            schema: RefOr::Item(Schema::of_desc(
                &desc,
                "MediaType".to_string(),
                None,
                context,
            )),
        }
    }
}
