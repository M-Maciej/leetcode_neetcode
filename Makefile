.PHONY: rust python cpp
# To run specific problem like contains_duplicate, run : make rust p=constains_duplicate
rust:
	cd rust && cargo test --bin $(p) -- --nocapture
python:
	./.venv/bin/python3 python/$(p).py
cpp:
	mkdir -p cpp/bin
	clang++ -std=c++20 cpp/$(p).cpp -o cpp/bin/$(p)_out
	./cpp/bin/$(p)_out
