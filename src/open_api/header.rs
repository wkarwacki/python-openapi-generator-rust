
use serde::{Deserialize, Serialize};




use crate::open_api::schema::Schema;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Header {
    pub schema: Schema,
}
