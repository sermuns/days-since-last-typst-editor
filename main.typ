#let releases = csv("data.csv", row-type: dictionary).sorted(key: it => it.date).rev()

#document("dist/index.html", {
  html.html({
    html.head({
      html.meta(charset: "utf-8")
      html.meta(name: "viewport", content: "width=initial-width")
      html.link(rel: "icon", href: "/favicon.png")
      html.title[Days since last Typst editor]
      html.style(read("style.css"))
    })
    html.body({
      html.div(style: "display:flex;justify-content:space-between;", {
        html.a(
          target: "_blank",
          href: "https://codeberg.org/noClaps/days-since-last-typst-editor",
        )[source code for this website]
        html.span(style: "text-align:right;", {
          "created by "
          html.a(target: "_blank", href: "https://github.com/sermuns")[Samuel "sermuns" Åkesson]
        })
      })
      html.h3[It has been]
      html.elem("h2", attrs: (id: "days", data-last-release: (releases.first().date)), {
        html.noscript[? (javascript is disabled)]
      })
      html.h3(id: "sentenceEnd")[days since the last release of a Typst editor]
      html.div(id: "timeline", {
        for release in releases {
          html.i(style: "text-align:right;", release.name)
          html.span[was released #release.date]
          if release.source_code_url != "" {
            html.a(href: release.source_code_url, "source")
          } else {
            html.div()
          }
          if release.reddit_url != "" {
            html.a(href: release.reddit_url, "reddit")
          } else {
            html.div()
          }
          if release.typst_forum_url != "" {
            html.a(href: release.typst_forum_url, "forum")
          } else {
            html.div()
          }
        }
      })
      html.script(read("script.js"))
    })
  })
})
#asset("dist/favicon.png", read("favicon.png", encoding: none))
