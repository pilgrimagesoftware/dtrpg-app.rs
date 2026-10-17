.PHONY: size-check

MAX_LINES := 500

# Enforces AGENTS.md's "no .rs file over 500 lines" rule. Excludes
# tests/examples/benches directories (none exist today -- this repo's own
# tests live inline as #[cfg(test)] modules -- but the rule is scoped that
# way if one is ever added) and target/.
size-check:
	@violations=$$(find crates -name '*.rs' \
		-not -path '*/target/*' \
		-not -path '*/tests/*' \
		-not -path '*/examples/*' \
		-not -path '*/benches/*' \
		-exec sh -c 'lines=$$(wc -l < "$$1"); if [ "$$lines" -gt $(MAX_LINES) ]; then echo "$$lines $$1"; fi' _ {} \;); \
	if [ -n "$$violations" ]; then \
		echo "Files over $(MAX_LINES) lines:"; \
		echo "$$violations" | sort -rn; \
		exit 1; \
	fi
