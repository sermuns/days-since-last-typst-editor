use maud::{DOCTYPE, html};
use std::{fs, io, path::Path};

use crate::data::Release;

pub fn render(releases: &[Release], output_dir: impl AsRef<Path>) -> io::Result<()> {
    let rendered = html! {
        @for release in releases {
            b { (release.name) }
        }
    };

    let output_dir = output_dir.as_ref();
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    let output_file = output_dir.join("index.html");
    fs::write(&output_file, rendered.into_string())?;

    println!("rendered to {}", output_file.display());

    Ok(())
}
