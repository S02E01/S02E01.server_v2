.PHONY: watch
watch: 
	systemfd --no-pid -s http::4000 -- cargo watch -x run


.PHONY: run
run: 
	cargo run

.PHONY: build
build: 
	cargo build


.PHONY: count
count: 
	find ./src -name tests -prune -o -type f -name '*.rs' | xargs wc -l


.DEFAULT_GOAL := watch