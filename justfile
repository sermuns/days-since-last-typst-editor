[parallel]
watch-and-serve: watch serve

watch:
    watchexec -i dist -- cargo run

serve:
    penguin serve dist
