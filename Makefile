# To run specific problem like contains_duplicate, run : make rust p=constains_duplicate
rust:
	cargo -C rust/ run --bin $(p)
python:
	source .venv/bin/activate
	python3 python/$(p).py
cpp:
	mkdir -p cpp/bin
	clang++ -std=c++20 cpp/$(p).cpp -o cpp/bin/$(p)_out
	./cpp/bin/$(p)_out
