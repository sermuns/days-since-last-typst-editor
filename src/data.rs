use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub releases: Vec<Release>,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub name: String,
    pub date: String,
}
