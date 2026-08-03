use std::{fs, io, path::Path};

use maud::{DOCTYPE, PreEscaped, html};

struct Release {
    date: String,
    name: String,
    reddit_url: String,
    source_code_url: String,
}

fn main() -> io::Result<()> {
    let mut releases = Vec::new();

    let mut csv_reader = csv::Reader::from_path("data.csv")?;
    for result in csv_reader.records() {
        let record = result?;
        let mut fields = record.into_iter();

        releases.push(Release {
            date: fields.next().unwrap().to_owned(),
            name: fields.next().unwrap().to_owned(),
            reddit_url: fields.next().unwrap().to_owned(),
            source_code_url: fields.next().unwrap().to_owned(),
        });
    }

    releases.sort_by(|a, b| b.date.cmp(&a.date));

    render(&releases, "dist")
}

fn render(releases: &[Release], output_dir: impl AsRef<Path>) -> io::Result<()> {
    let rendered = html! {
        (DOCTYPE)

        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="icon" href="/favicon.png";
            title { "Days since last Typst editor" }
            style { (fs::read_to_string("style.css")?) }
        }

        body {
            div style="display:flex;justify-content:space-between;" {
                a target="_blank" href=(env!("CARGO_PKG_REPOSITORY")) {
                    "source code for this website"
                }
                span {
                    "created by "
                    a target="_blank" href="https://github.com/sermuns" {
                        r#"Samuel "sermuns" Åkesson"#
                    }
                }
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
                    date, name, reddit_url, source_code_url,
                } in releases {
                    i style="text-align:right" { (name) }

                    span {
                        " was released "
                        (date)
                    }

                    @if !reddit_url.is_empty() {
                        a href=(reddit_url) { "reddit" }
                    } @else {
                        div {}
                    }

                    @if !source_code_url.is_empty() {
                        a href=(source_code_url) { "source" }
                    } @else {
                        div {}
                    }
                }
            }

            script { (PreEscaped(fs::read_to_string("script.js")?)) }
        }
    };

    let output_dir = output_dir.as_ref();
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    let output_file = output_dir.join("index.html");
    fs::write(&output_file, rendered.into_string())?;

    fs::copy("favicon.png", output_dir.join("favicon.png"))?;

    println!("rendered to {}", output_file.display());

    Ok(())
}
