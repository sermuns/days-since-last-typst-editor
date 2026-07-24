use std::{fs, io};

use crate::data::Release;

mod data;
mod site;

fn main() -> io::Result<()> {
    let mut releases = Vec::new();

    let mut csv_reader = csv::Reader::from_path("data.csv")?;
    for result in csv_reader.records() {
        let record = result?;
        let mut fields = record.into_iter();

        releases.push(Release {
            name: fields.next().unwrap().to_owned(),
            url: fields.next().unwrap().to_owned(),
            date: fields.next().unwrap().to_owned(),
        });
    }

    releases.sort();

    site::render(&releases, "dist")
}
