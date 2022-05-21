clean:
	cargo clean
	rm -rf *~ dist *.egg-info build target

build-pyo3:
	maturin build -i python3 --release --cargo-extra-args="--features python"

develop-pyo3:
	maturin develop --release --cargo-extra-args="--features python"

test:
	cargo test 

bench:
	cargo bench
