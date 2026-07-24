window.onload = () => {
	const now = new Date()
	const last_release = new Date(days.dataset.lastRelease)
	const days_since_last_release = Math.floor((now - last_release) / (1000 * 3600 * 24))
	days.textContent = days_since_last_release
}
