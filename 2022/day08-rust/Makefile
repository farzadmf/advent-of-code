SHELL := /bin/zsh
.ONESHELL:

p1-watch:
	cargo watch -x 'run --bin p1'

p2-watch:
	cargo watch -x 'run --bin p2'

get-input:
	[[ $$AOC_COOKIE ]] || { echo 'set $$AOC_COOKIE'; return 1 }
	curl -sL --cookie "session=$$AOC_COOKIE" https://adventofcode.com/2022/day/8/input > input && \
		echo 'input saved?' || echo 'failed!'
