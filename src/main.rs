use std::{fs, io};

use crate::data::Data;

mod data;
mod site;

fn main() -> io::Result<()> {
    let data_bytes = fs::read("data.toml")?;
    let Data { mut releases } = toml::from_slice(&data_bytes).unwrap();
    releases.sort_by(|a, b| a.name.cmp(&b.name));

    dbg!(&releases);

    site::render(&releases, "dist")
}
