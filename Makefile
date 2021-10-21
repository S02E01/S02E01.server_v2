.PHONY: count

count:
	find ./src -name tests -prune -o -type f -name '*.rs' | xargs wc -l