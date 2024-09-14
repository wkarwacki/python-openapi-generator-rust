use crate::{HashMap, Path};
use rust_embed::{Embed, RustEmbed};

pub trait Templates: RustEmbed {
    fn default(&self) -> HashMap<String, String> {
        Self::iter()
            .flat_map(|str| {
                Self::get(str.clone().into_owned().as_str())
                    .iter()
                    .map(|embedded_file| {
                        (
                            Path::new(str.clone().into_owned().as_str())
                                .file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                            std::str::from_utf8(embedded_file.data.as_ref())
                                .unwrap()
                                .to_string(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>()
    }
}
