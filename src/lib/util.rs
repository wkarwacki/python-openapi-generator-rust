use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub fn read(path: PathBuf) -> String {
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn read_t<T>(path: PathBuf) -> T
where
    T: DeserializeOwned,
{
    let content = read(path);
    serde_yaml::from_str(&content).unwrap()
}

pub fn write<T>(t: T, path: PathBuf)
where
    T: Serialize,
{
    let out = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    serde_yaml::to_writer(out, &t).unwrap();
}

pub fn val(b: &bool) -> bool {
    *b == true
}

pub fn r#true() -> bool {
    true
}
