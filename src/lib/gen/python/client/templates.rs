use crate::lib::gen::templates::Templates as GenTemplates;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/src/lib/gen/python/client/templates"]
pub struct Templates;

impl GenTemplates for Templates {}
