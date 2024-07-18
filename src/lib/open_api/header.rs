use crate::lib::open_api::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Header {
    pub schema: Schema,
}
