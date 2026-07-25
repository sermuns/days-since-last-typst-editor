use std::{fs, io, path::Path};

use maud::{DOCTYPE, PreEscaped, html};

struct Release {
    date: String,
    url: String,
    name: String,
}

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

    releases.sort_by(|a, b| b.date.cmp(&a.date));

    render(&releases, "dist")
}

fn render(releases: &[Release], output_dir: impl AsRef<Path>) -> io::Result<()> {
    let style_contents = fs::read_to_string("style.css")?;
    let script_contents = fs::read_to_string("script.js")?;

    let rendered = html! {
        (DOCTYPE)

        title { "Days since last Typst editor"}

        style { (style_contents) }

        div id="top" {
            a target="_blank" href=(env!("CARGO_PKG_REPOSITORY")) { "source code" }
        }

        h3 {
            "It has been"
        }
        h2 id="days" data-last-release=(releases.first().unwrap().date) {
            noscript {
                "? (javascript is disabled)"
            }
        }
        h3 id="sentenceEnd" {
            "days since the last release of a Typst editor"
        }

        div id="timeline" {
            @for Release {
                name, url, date
            } in releases {
                a href=(url) { (name) }
                span { " released " (date) }
            }
        }

        script { (PreEscaped(script_contents)) }
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
