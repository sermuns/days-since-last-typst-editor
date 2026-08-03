async function fetch_num_github_stars(url) {
	const res = await fetch(`https://api.github.com/repos/${project}`)
	const json = await res.json()
	if (json.errors) throw new Error(json.errors[0].message)
	return json["stargazers_count"]
}

async function fetch_num_codeberg_stars(url) {
	// TODO:
}

window.onload = async () => {
	const now = new Date()
	const last_release = new Date(days.dataset.lastRelease)
	const days_since_last_release = Math.floor((now - last_release) / (1000 * 3600 * 24))
	days.textContent = days_since_last_release
	sentenceEnd.textContent = `day${days_since_last_release == 1 ? "" : "s"} since the last release of a Typst editor`

	for (a of document.querySelectorAll('#timeline > a')) {
		const url = a.href
		if (url.includes("reddit.com/")) continue
		if (url.includes("github.com")) {
		}
	}

}
