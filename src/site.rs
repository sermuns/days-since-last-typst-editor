use maud::{DOCTYPE, html};
use std::{fs, io, path::Path};

use crate::data::Release;

pub fn render(releases: &[Release], output_dir: impl AsRef<Path>) -> io::Result<()> {
    let days_since_last_release = 30;

    let style_contents = fs::read_to_string("style.css")?;

    let rendered = html! {
        style { (style_contents) }

        h3 {
            "It has been"
        }
        h2 {
            (days_since_last_release)
        }
        h3 {
            "days since the last release of a Typst editor"
        }

        div id="timeline" {
            @for Release {
                name, url, date
            } in releases {
                a href=(url) { (name) }
                span { " was released on " (date) }
            }
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
