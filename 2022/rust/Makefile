SHELL := /bin/zsh
.ONESHELL:

define watch_problem
	[[ -n "$(d)" ]] || { echo "specify 'd=...' for the day"; return 1 }
	local padded_day=$$( python -c "print('$(d)'.zfill(2), end=None)" ) && \
		cargo watch -x "run --bin day$${padded_day}_p$(1)"
endef

p1-watch:
	$(call watch_problem,1)

p2-watch:
	$(call watch_problem,2)

get-input:
	[[ $$AOC_COOKIE ]] || { echo 'set $$AOC_COOKIE'; return 1 }
	curl -sL --cookie "session=$$AOC_COOKIE" __aoc_url__ > input && \
		echo 'input saved?' || echo 'failed!'
